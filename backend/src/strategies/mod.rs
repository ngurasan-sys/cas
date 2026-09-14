use crate::event_bus::MarketEvent;
use crate::risk::OrderIntent;

pub trait Strategy {
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<OrderIntent>;
    fn name(&self) -> &'static str;
}

pub struct StrategyRouter {
    strategies: Vec<Box<dyn Strategy>>,
}

impl StrategyRouter {
    pub fn new() -> Self {
        Self { strategies: Vec::new() }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategies.push(strategy);
    }
}
