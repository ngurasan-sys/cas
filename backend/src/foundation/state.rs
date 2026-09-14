use std::sync::Arc;

// We need to use the concrete EventBus implementation, not a trait, or import the right one.
// Let's use the concrete struct from event_bus::mod
use crate::event_bus::EventBus;

use crate::market_data::SubscriptionManager;
use std::sync::RwLock;

pub struct AppState {
    pub event_bus: Arc<EventBus>,
    pub subscription_manager: Arc<RwLock<SubscriptionManager>>,
    // Future: DB pools, Config, etc.
}

impl AppState {
    pub fn new(
        event_bus: Arc<EventBus>,
        subscription_manager: Arc<RwLock<SubscriptionManager>>,
    ) -> Self {
        Self {
            event_bus,
            subscription_manager,
        }
    }
}
