use std::{sync::Arc, time::Duration};

use agentdb_system::*;
use rand::random;
use serde::{Deserialize, Serialize};

use super::callback::{EffectCallback, EffectContext};
use super::{EffectFailure, EffectFailureReason};

#[derive(Serialize, Deserialize)]
pub struct RetryBackoff {
    initial_backoff: Duration,
    base: f64,
    jitter: f64,
    exponent: f64,
}

impl RetryBackoff {
    pub fn calculate_delay(&self, attempt: u64) -> Duration {
        let attempt = attempt as f64;
        let exp_delay = self.base.powf(attempt);
        let poly_delay = (attempt + 1.0).powf(self.exponent);
        let perfect_delay = exp_delay * poly_delay;
        let jitter_factor = (random::<f64>() * 2.0 - 1.0) * self.jitter + 1.0;
        self.initial_backoff.mul_f64(perfect_delay * jitter_factor)
    }
    pub fn exponential(base: f64, initial_backoff: Duration) -> Self {
        Self {
            initial_backoff,
            base,
            jitter: 0.0,
            exponent: 0.0,
        }
    }
    pub fn with_jitter(mut self, jitter: f64) -> Self {
        self.jitter = jitter;
        self
    }
    pub fn polynomial(exponent: f64, initial_backoff: Duration) -> Self {
        Self {
            initial_backoff,
            base: 1.0,
            jitter: 0.0,
            exponent,
        }
    }
    pub fn linear(initial_backoff: Duration) -> Self {
        Self::polynomial(1.0, initial_backoff)
    }
    pub fn quadratic(initial_backoff: Duration) -> Self {
        Self::polynomial(2.0, initial_backoff)
    }
}

// Effect agent which will automatically retry a callback
#[agent(name = "adb.effects.retry")]
#[derive(Serialize, Deserialize)]
pub struct Retry {
    config: DoRetry,
    attempt: u64,
}

impl Retry {
    /// Returns true if the agent should delete itself
    async fn trigger(
        &mut self,
        ref_: AgentRef<Self>,
        context: &mut Context<'_>,
    ) -> Result<bool, Error> {
        let attempt = self.attempt;

        // Check max attempts
        if let Some(max_attempts) = self.config.max_attempts {
            if attempt >= max_attempts {
                // Max attempts reached, send a failure response and
                // delete ourselves.
                context.dyn_send(
                    self.config.caller,
                    Box::new(EffectFailure {
                        ref_: ref_.into(),
                        reason: EffectFailureReason::MaxAttemptsExceeded,
                    }),
                )?;
                return Ok(true);
            }
        }

        // Schedule next retry
        let delay = self.config.backoff.calculate_delay(attempt);
        context.send_at(ref_, TriggerRetry, Timestamp::now() + delay)?;

        // Register callback
        let callback = self.config.callback.clone();
        context.run_on_commit(move |hc| {
            callback.call(EffectContext {
                ref_: ref_.into(),
                inner: hc,
                attempt,
            })
        });

        self.attempt += 1;
        Ok(false)
    }
}

// Message to construct a retry agent
#[message(name = "adb.effects.retry.do")]
#[derive(Serialize, Deserialize)]
pub struct DoRetry {
    caller: DynAgentRef,
    backoff: RetryBackoff,
    max_attempts: Option<u64>,
    timeout: Option<Duration>,
    callback: Arc<dyn EffectCallback>,
}

impl DoRetry {
    pub fn new(caller: impl Into<DynAgentRef>, callback: impl EffectCallback) -> Self {
        Self {
            caller: caller.into(),
            callback: Arc::new(callback),
            backoff: RetryBackoff::exponential(2.0, Duration::from_secs(5)),
            timeout: None,
            max_attempts: Some(5),
        }
    }
    pub fn with_backoff(mut self, backoff: RetryBackoff) -> Self {
        self.backoff = backoff;
        self
    }
    pub fn with_max_attempts(mut self, max_attempts: Option<u64>) -> Self {
        self.max_attempts = max_attempts;
        self
    }
    pub fn with_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.timeout = timeout;
        self
    }
}

#[constructor]
impl Construct for DoRetry {
    type Agent = Retry;

    async fn construct(
        self,
        ref_: AgentRef<Retry>,
        context: &mut Context<'_>,
    ) -> Result<Option<Retry>, Error> {
        let mut agent = Retry {
            config: self,
            attempt: 0,
        };
        if agent.trigger(ref_, context).await? {
            Ok(None)
        } else {
            // If there's an overall timeout then schedule that
            if let Some(timeout) = agent.config.timeout {
                context.dyn_send_at(
                    ref_.into(),
                    Box::new(EffectFailure {
                        ref_: ref_.into(),
                        reason: EffectFailureReason::TimedOut,
                    }),
                    Timestamp::now() + timeout,
                )?;
            }
            Ok(Some(agent))
        }
    }
}

#[message(name = "adb.effects.retry.trigger")]
#[derive(Serialize, Deserialize)]
struct TriggerRetry;

#[handler]
impl Handle<TriggerRetry> for Retry {
    async fn handle(
        &mut self,
        ref_: AgentRef<Self>,
        _msg: TriggerRetry,
        context: &mut Context,
    ) -> Result<bool, Error> {
        self.trigger(ref_, context).await
    }
}

// Forward any unknown messages to the caller and delete ourselves
#[handler]
impl HandleDyn for Retry {
    async fn handle_dyn(
        &mut self,
        _ref_: AgentRef<Self>,
        message: DynMessage,
        context: &mut Context,
    ) -> Result<bool, Error> {
        context.dyn_send(self.config.caller, message)?;
        Ok(true)
    }
}
