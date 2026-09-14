use std::sync::Arc;

// We need to use the concrete EventBus implementation, not a trait, or import the right one.
// Let's use the concrete struct from event_bus::mod
use crate::event_bus::EventBus;

pub struct AppState {
    pub event_bus: Arc<EventBus>,
    // Future: DB pools, Config, etc.
}

impl AppState {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}
