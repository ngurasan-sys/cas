use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationId(pub String);

impl CorrelationId {
    pub fn new() -> Self {
        // Simple random correlation ID for now
        let id = format!("{:016x}", rand::random::<u64>());
        Self(id)
    }
}

// In rust, timestamps are often passed as u64 millis since epoch or chrono DateTimes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTimestamp {
    pub event_ts: i64,
    pub receive_ts: i64,
    pub source_ts: Option<i64>,
}
