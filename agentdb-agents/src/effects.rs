use agentdb_system::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

pub mod callback;
pub mod retry;
