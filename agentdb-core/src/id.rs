//! Module for generating V1 UUIDs
use chrono::Utc;
use lazy_static::lazy_static;
use rand::RngCore;
use uuid::{v1, Uuid};

/// Generate a new V1 UUID
pub fn new() -> Uuid {
    // A static instance of the Generator so consumers can call `uuid::new_v1()` directly if they
    // just want a single process-wide Generator.
    lazy_static! {
        static ref GENERATOR: Generator = Generator::default();
    }

    GENERATOR.new_v1()
}

struct Generator {
    context: v1::Context,
    node_id: [u8; 6],
}

impl Default for Generator {
    fn default() -> Self {
        let mut node_id: [u8; 6] = [0u8; 6];
        rand::thread_rng().fill_bytes(&mut node_id);

        Generator {
            context: v1::Context::new(0),
            node_id,
        }
    }
}

impl Generator {
    /// Generate a v1, time-based UUID using the system clock and the Generator's Node ID.
    pub fn new_v1(&self) -> Uuid {
        let now = Utc::now();
        Uuid::new_v1(
            v1::Timestamp::from_unix(
                &self.context,
                now.timestamp() as u64,
                now.timestamp_subsec_nanos(),
            ),
            &self.node_id,
        )
        .expect("Failed to generate V1 UUID")
    }
}
