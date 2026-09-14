use backend::instruments::{InstrumentMaster, InMemoryInstrumentMaster, Instrument, InstrumentType};

fn setup_master() -> InMemoryInstrumentMaster {
    let mut master = InMemoryInstrumentMaster::new();

    // Add Spot
    master.add_instrument(Instrument {
        instrument_id: "NIFTY_SPOT".into(),
        symbol: "NIFTY 50".into(),
        trading_symbol: "NIFTY 50".into(),
        exchange: "NSE".into(),
        segment: "IDX".into(),
        underlying: "NIFTY".into(),
        instrument_type: InstrumentType::Spot,
        expiry: None,
        strike: None,
        lot_size: 1,
        tick_size: 0.05,
        status: "ACTIVE".into(),
        broker_security_id: "256265".into(),
        version: 1,
    });

    // Add Future
    master.add_instrument(Instrument {
        instrument_id: "NIFTY_FUT_1".into(),
        symbol: "NIFTY FUT".into(),
        trading_symbol: "NIFTY24OCTFUT".into(),
        exchange: "NFO".into(),
        segment: "FUTIDX".into(),
        underlying: "NIFTY".into(),
        instrument_type: InstrumentType::Future,
        expiry: Some(1730332800), // Some dummy unix timestamp
        strike: None,
        lot_size: 25,
        tick_size: 0.05,
        status: "ACTIVE".into(),
        broker_security_id: "12345".into(),
        version: 1,
    });

    // Add CE Option
    master.add_instrument(Instrument {
        instrument_id: "NIFTY_25000_CE".into(),
        symbol: "NIFTY CE".into(),
        trading_symbol: "NIFTY24OCT25000CE".into(),
        exchange: "NFO".into(),
        segment: "OPTIDX".into(),
        underlying: "NIFTY".into(),
        instrument_type: InstrumentType::OptionCE,
        expiry: Some(1730332800),
        strike: Some(25000.0),
        lot_size: 25,
        tick_size: 0.05,
        status: "ACTIVE".into(),
        broker_security_id: "12346".into(),
        version: 1,
    });

    master
}

#[test]
fn test_instrument_master_lookups() {
    let master = setup_master();

    let spot = master.get_instrument("NIFTY_SPOT").unwrap();
    assert_eq!(spot.symbol, "NIFTY 50");

    let by_symbol = master.get_by_symbol("NIFTY FUT").unwrap();
    assert_eq!(by_symbol.instrument_id, "NIFTY_FUT_1");

    let nifty_instruments = master.get_by_underlying("NIFTY");
    assert_eq!(nifty_instruments.len(), 3);

    let futures = master.get_futures("NIFTY");
    assert_eq!(futures.len(), 1);
    assert_eq!(futures[0].instrument_type, InstrumentType::Future);

    let options = master.get_options("NIFTY", 1730332800, InstrumentType::OptionCE);
    assert_eq!(options.len(), 1);
    assert_eq!(options[0].strike, Some(25000.0));
}
