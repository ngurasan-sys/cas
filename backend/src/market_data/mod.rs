use crate::event_bus::{EventBus, MarketEvent};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};

pub trait MarketDataGateway {
    fn subscribe(&self, instrument_id: &str);
    fn unsubscribe(&self, instrument_id: &str);
}

pub struct SyntheticDataGenerator {
    event_bus: Arc<EventBus>,
}

impl SyntheticDataGenerator {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }

    pub fn start(&self) {
        let bus = self.event_bus.clone();
        tokio::spawn(async move {
            let mut price_nifty = 25100.0;
            let mut price_banknifty = 51200.0;

            loop {
                // Simulate some random movement
                price_nifty += (rand::random::<f64>() - 0.5) * 5.0;
                price_banknifty += (rand::random::<f64>() - 0.5) * 10.0;

                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64;

                bus.publish(MarketEvent {
                    instrument_id: "NIFTY".to_string(),
                    price: price_nifty,
                    timestamp,
                });

                bus.publish(MarketEvent {
                    instrument_id: "BANKNIFTY".to_string(),
                    price: price_banknifty,
                    timestamp,
                });

                sleep(Duration::from_millis(500)).await;
            }
        });
    }
}
