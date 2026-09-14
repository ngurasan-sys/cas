use crate::event_bus::{MarketDataType, MarketEvent};
use crate::foundation::types::EventTimestamp;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};

use crate::market_data::gateway::MarketDataGateway;
use std::sync::Mutex;

pub struct SyntheticProvider {
    gateway: Arc<Mutex<MarketDataGateway>>,
    seq_num: u64,
}

impl SyntheticProvider {
    pub fn new(gateway: Arc<Mutex<MarketDataGateway>>) -> Self {
        Self {
            gateway,
            seq_num: 0,
        }
    }

    fn generate_event(&mut self, id: &str, price: f64) -> MarketEvent {
        self.seq_num += 1;
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        MarketEvent {
            instrument_id: id.to_string(),
            data_type: MarketDataType::Tick,
            timestamp: EventTimestamp {
                event_ts: ts,
                receive_ts: ts,
                source_ts: Some(ts),
            },
            sequence: Some(self.seq_num),
            source: "SYNTHETIC".to_string(),
            price,
            volume: Some((rand::random::<f64>() * 100.0) as u64),
            bid: Some(price - 0.5),
            ask: Some(price + 0.5),
            open_interest: None,
            iv: None,
            delta: None,
            gamma: None,
        }
    }

    pub fn start(&mut self) {
        let gw = self.gateway.clone();
        tokio::spawn(async move {
            let mut price_nifty = 25100.0;
            let mut price_banknifty = 51200.0;
            let mut price_sensex = 81000.0;
            let mut provider = SyntheticProvider::new(gw.clone()); // Shadow copy for the task

            loop {
                price_nifty += (rand::random::<f64>() - 0.5) * 5.0;
                price_banknifty += (rand::random::<f64>() - 0.5) * 10.0;
                price_sensex += (rand::random::<f64>() - 0.5) * 15.0;

                let events = vec![
                    provider.generate_event("NIFTY_SPOT", price_nifty),
                    provider.generate_event("BANKNIFTY_SPOT", price_banknifty),
                    provider.generate_event("SENSEX_SPOT", price_sensex),
                    provider.generate_event("NIFTY_FUT_1", price_nifty + 50.0),
                    provider
                        .generate_event("NIFTY_25000_CE", 150.0 + (rand::random::<f64>() * 10.0)),
                ];

                if let Ok(mut gw_lock) = provider.gateway.lock() {
                    for event in events {
                        let _ = gw_lock.process_event(event);
                    }
                }

                sleep(Duration::from_millis(500)).await;
            }
        });
    }
}
