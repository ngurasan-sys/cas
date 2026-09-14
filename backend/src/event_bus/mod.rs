use crate::foundation::types::EventTimestamp;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarketDataType {
    Tick,
    Quote, // Bid/Ask updates
    Depth, // Orderbook depth updates
    Greeks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketEvent {
    pub instrument_id: String,
    pub data_type: MarketDataType,

    // Explicit timestamping model as per foundation
    pub timestamp: EventTimestamp,

    pub sequence: Option<u64>,
    pub source: String,

    pub price: f64,
    pub volume: Option<u64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,

    // Option specific
    pub open_interest: Option<u64>,
    pub iv: Option<f64>,
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
}

pub struct EventBus {
    sender: broadcast::Sender<MarketEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(4096); // Bounded channel for backpressure
        Self { sender }
    }

    pub fn publish(&self, event: MarketEvent) {
        let _ = self.sender.send(event); // Discard errors if no active receivers exist
    }

    pub fn subscribe(&self) -> broadcast::Receiver<MarketEvent> {
        self.sender.subscribe()
    }
}
