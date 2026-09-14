pub struct OrderIntent {
    pub instrument_id: String,
    pub quantity: i32,
    pub order_type: String, // E.g. BUY, SELL
}

pub trait RiskEngine {
    fn validate_order(&self, intent: &OrderIntent) -> Result<(), String>;
}
