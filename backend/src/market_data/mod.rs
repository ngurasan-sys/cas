use crate::event_bus::MarketEvent;

pub trait MarketDataGateway {
    fn subscribe(&self, instrument_id: &str);
    fn unsubscribe(&self, instrument_id: &str);
    // Real implementation would connect to broker adapters
}
