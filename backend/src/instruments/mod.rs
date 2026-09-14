use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub trading_symbol: String,
    pub exchange: String,
    pub segment: String,
    pub underlying: String,
    pub instrument_type: InstrumentType,
    pub expiry: Option<i64>, // Unix timestamp
    pub strike: Option<f64>,
    pub lot_size: u32,
    pub tick_size: f64,
    pub status: String,
    pub broker_security_id: String,
    pub version: u32,
}

pub trait InstrumentMaster {
    fn get_instrument(&self, instrument_id: &str) -> Option<Instrument>;
    fn add_instrument(&mut self, instrument: Instrument);
    fn update_instrument(&mut self, instrument: Instrument);
    fn get_by_symbol(&self, symbol: &str) -> Option<Instrument>;
    fn get_by_underlying(&self, underlying: &str) -> Vec<Instrument>;
    fn get_options(
        &self,
        underlying: &str,
        expiry: i64,
        option_type: InstrumentType,
    ) -> Vec<Instrument>;
    fn get_futures(&self, underlying: &str) -> Vec<Instrument>;
    fn get_by_strike(&self, underlying: &str, strike: f64) -> Vec<Instrument>;
    fn get_exact_option(
        &self,
        underlying: &str,
        expiry: i64,
        option_type: InstrumentType,
        strike: f64,
    ) -> Option<Instrument>;
}

pub struct InMemoryInstrumentMaster {
    instruments: Arc<RwLock<HashMap<String, Instrument>>>,
}

impl InMemoryInstrumentMaster {
    pub fn new() -> Self {
        Self {
            instruments: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl InstrumentMaster for InMemoryInstrumentMaster {
    fn get_instrument(&self, instrument_id: &str) -> Option<Instrument> {
        let lock = self.instruments.read().unwrap();
        lock.get(instrument_id).cloned()
    }

    fn add_instrument(&mut self, instrument: Instrument) {
        let mut lock = self.instruments.write().unwrap();
        lock.insert(instrument.instrument_id.clone(), instrument);
    }

    fn update_instrument(&mut self, instrument: Instrument) {
        self.add_instrument(instrument);
    }

    fn get_by_symbol(&self, symbol: &str) -> Option<Instrument> {
        let lock = self.instruments.read().unwrap();
        lock.values().find(|inst| inst.symbol == symbol).cloned()
    }

    fn get_by_underlying(&self, underlying: &str) -> Vec<Instrument> {
        let lock = self.instruments.read().unwrap();
        lock.values()
            .filter(|inst| inst.underlying == underlying)
            .cloned()
            .collect()
    }

    fn get_options(
        &self,
        underlying: &str,
        expiry: i64,
        option_type: InstrumentType,
    ) -> Vec<Instrument> {
        let lock = self.instruments.read().unwrap();
        lock.values()
            .filter(|inst| {
                inst.underlying == underlying
                    && inst.instrument_type == option_type
                    && inst.expiry == Some(expiry)
            })
            .cloned()
            .collect()
    }

    fn get_futures(&self, underlying: &str) -> Vec<Instrument> {
        let lock = self.instruments.read().unwrap();
        let mut futures: Vec<Instrument> = lock
            .values()
            .filter(|inst| {
                inst.underlying == underlying && inst.instrument_type == InstrumentType::Future
            })
            .cloned()
            .collect();
        // Sort futures by expiry
        futures.sort_by_key(|a| a.expiry.unwrap_or(i64::MAX));
        futures
    }

    fn get_by_strike(&self, underlying: &str, strike: f64) -> Vec<Instrument> {
        let lock = self.instruments.read().unwrap();
        lock.values()
            .filter(|inst| inst.underlying == underlying && inst.strike == Some(strike))
            .cloned()
            .collect()
    }

    fn get_exact_option(
        &self,
        underlying: &str,
        expiry: i64,
        option_type: InstrumentType,
        strike: f64,
    ) -> Option<Instrument> {
        let lock = self.instruments.read().unwrap();
        lock.values()
            .find(|inst| {
                inst.underlying == underlying
                    && inst.instrument_type == option_type
                    && inst.expiry == Some(expiry)
                    && inst.strike == Some(strike)
            })
            .cloned()
    }
}
