use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketEvent {
    pub instrument_id: String,
    pub price: f64,
    pub timestamp: i64,
}

pub trait EventBus {
    fn publish(&self, event: MarketEvent);
    // Real implementation would involve async channels or pub/sub
}
