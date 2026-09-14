use backend::instruments::{
    InMemoryInstrumentMaster, Instrument, InstrumentMaster, InstrumentType,
};

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

    // Add PE Option (same expiry, diff strike)
    master.add_instrument(Instrument {
        instrument_id: "NIFTY_25100_PE".into(),
        symbol: "NIFTY PE".into(),
        trading_symbol: "NIFTY24OCT25100PE".into(),
        exchange: "NFO".into(),
        segment: "OPTIDX".into(),
        underlying: "NIFTY".into(),
        instrument_type: InstrumentType::OptionPE,
        expiry: Some(1730332800),
        strike: Some(25100.0),
        lot_size: 25,
        tick_size: 0.05,
        status: "ACTIVE".into(),
        broker_security_id: "12347".into(),
        version: 1,
    });

    // Add CE Option (diff expiry)
    master.add_instrument(Instrument {
        instrument_id: "NIFTY_25000_CE_NEXT".into(),
        symbol: "NIFTY CE NEXT".into(),
        trading_symbol: "NIFTY24NOV25000CE".into(),
        exchange: "NFO".into(),
        segment: "OPTIDX".into(),
        underlying: "NIFTY".into(),
        instrument_type: InstrumentType::OptionCE,
        expiry: Some(1732924800),
        strike: Some(25000.0),
        lot_size: 25,
        tick_size: 0.05,
        status: "ACTIVE".into(),
        broker_security_id: "12348".into(),
        version: 1,
    });

    master
}

#[test]
fn test_instrument_master_lookups() {
    let master = setup_master();

    let spot = master.get_instrument("NIFTY_SPOT").unwrap();
    assert_eq!(spot.symbol, "NIFTY 50");

    // Negative lookup test
    assert!(master.get_instrument("NON_EXISTENT").is_none());

    let by_symbol = master.get_by_symbol("NIFTY FUT").unwrap();
    assert_eq!(by_symbol.instrument_id, "NIFTY_FUT_1");

    let nifty_instruments = master.get_by_underlying("NIFTY");
    // Spot, Fut, and 3 options
    assert_eq!(nifty_instruments.len(), 5);

    let futures = master.get_futures("NIFTY");
    assert_eq!(futures.len(), 1);
    assert_eq!(futures[0].instrument_type, InstrumentType::Future);

    // Option lookups
    let ce_options = master.get_options("NIFTY", 1730332800, InstrumentType::OptionCE);
    assert_eq!(ce_options.len(), 1);
    assert_eq!(ce_options[0].strike, Some(25000.0));

    let strike_options = master.get_by_strike("NIFTY", 25000.0);
    // Should return both the OCT and NOV CE options
    assert_eq!(strike_options.len(), 2);

    let exact_opt = master
        .get_exact_option("NIFTY", 1730332800, InstrumentType::OptionPE, 25100.0)
        .unwrap();
    assert_eq!(exact_opt.instrument_id, "NIFTY_25100_PE");

    // Negative exact lookup
    assert!(
        master
            .get_exact_option("NIFTY", 1730332800, InstrumentType::OptionCE, 99999.0)
            .is_none()
    );
}
