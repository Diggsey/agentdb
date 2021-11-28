//! Agents which manage external effects.

use agentdb_system::*;
use serde::{Deserialize, Serialize};

/// The possible reasons why a side-effectful operation may
/// have failed.
#[derive(Debug, Serialize, Deserialize)]
pub enum EffectFailureReason {
    /// The configured maximum number of attempts were reached.
    MaxAttemptsExceeded,
    /// The configured overall timeout was reached.
    TimedOut,
    /// A custom failure reason.
    Custom(String),
}

/// A failure message sent to a caller when a side-effectful operation
/// fails.
#[message(name = "adb.effects.failure")]
#[derive(Serialize, Deserialize)]
pub struct EffectFailure {
    ref_: DynAgentRef,
    reason: EffectFailureReason,
}

impl EffectFailure {
    /// The handle to the effect agent which failed.
    pub fn ref_(&self) -> DynAgentRef {
        self.ref_
    }
    /// The reason why the operation failed.
    pub fn reason(&self) -> &EffectFailureReason {
        &self.reason
    }
}

pub mod callback;
pub mod retry;
