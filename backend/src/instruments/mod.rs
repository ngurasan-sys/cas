use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstrumentType {
    Spot,
    Future,
    OptionCE,
    OptionPE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub instrument_id: String,
    pub symbol: String,
    pub instrument_type: InstrumentType,
    pub exchange: String,
    // Add other fields from requirements (lot_size, tick_size, etc.)
}

pub trait InstrumentMaster {
    fn get_instrument(&self, instrument_id: &str) -> Option<Instrument>;
    fn add_instrument(&mut self, instrument: Instrument);
}
