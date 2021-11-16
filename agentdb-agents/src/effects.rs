use agentdb_system::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EffectFailureReason {
    MaxAttemptsExceeded,
    TimedOut,
    Custom(String),
}

#[message(name = "adb.effects.failure")]
#[derive(Serialize, Deserialize)]
pub struct EffectFailure {
    ref_: DynAgentRef,
    reason: EffectFailureReason,
}

impl EffectFailure {
    pub fn ref_(&self) -> DynAgentRef {
        self.ref_
    }
    pub fn reason(&self) -> &EffectFailureReason {
        &self.reason
    }
}

pub mod callback;
pub mod retry;
