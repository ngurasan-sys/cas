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

/// Standardized timestamp model for the platform.
/// All timestamps are stored as milliseconds since the Unix Epoch (UTC).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTimestamp {
    /// Time when the event occurred/was generated.
    pub event_ts: i64,
    /// Time when our system received the event.
    pub receive_ts: i64,
    /// Time from the upstream source/exchange, if provided.
    pub source_ts: Option<i64>,
}
