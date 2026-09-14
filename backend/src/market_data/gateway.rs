use crate::event_bus::MarketEvent;
use async_trait::async_trait;

#[async_trait]
pub trait MarketDataProvider: Send + Sync {
    async fn connect(&mut self) -> Result<(), String>;
    async fn disconnect(&mut self) -> Result<(), String>;
    async fn subscribe(&mut self, instrument_ids: &[&str]) -> Result<(), String>;
    async fn unsubscribe(&mut self, instrument_ids: &[&str]) -> Result<(), String>;
    fn is_connected(&self) -> bool;
}

use crate::event_bus::EventBus;
use std::collections::HashMap;
use std::sync::Arc;

pub struct MarketDataGateway {
    // Track the last sequence number per source+instrument to detect duplicates/out-of-order
    last_sequence: HashMap<String, u64>,
    event_bus: Arc<EventBus>,
}

impl MarketDataGateway {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            last_sequence: HashMap::new(),
            event_bus,
        }
    }

    /// Process raw events from providers, applying stale/dup/malformed checks.
    /// If valid, routes it to the main EventBus.
    pub fn process_event(&mut self, event: MarketEvent) -> Result<(), &'static str> {
        if event.price <= 0.0 {
            return Err("Malformed data: price <= 0");
        }

        // Check for stale data (e.g., event is older than 5 seconds)
        if (event.timestamp.receive_ts - event.timestamp.event_ts) > 5000 {
            return Err("Stale data: event age exceeds 5 seconds");
        }

        // Duplicate/Sequence detection
        if let Some(seq) = event.sequence {
            let key = format!("{}_{}", event.source, event.instrument_id);
            if let Some(last_seq) = self.last_sequence.get(&key) {
                if seq <= *last_seq {
                    return Err("Duplicate or out-of-order sequence detected");
                }
            }
            self.last_sequence.insert(key, seq);
        }

        // Event passed validation, publish to the central event bus
        self.event_bus.publish(event);

        Ok(())
    }

    // Explicit validation-only helper for tests
    pub fn validate_event(&mut self, event: &MarketEvent) -> Result<(), &'static str> {
        self.process_event(event.clone())
    }
}
