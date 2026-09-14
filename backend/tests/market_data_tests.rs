use backend::event_bus::{EventBus, MarketDataType, MarketEvent};
use backend::foundation::types::EventTimestamp;
use backend::market_data::{MarketDataGateway, SubscriptionManager, SubscriptionTier};
use std::sync::Arc;

#[test]
fn test_gateway_stale_data_validation() {
    let bus = Arc::new(EventBus::new());
    let mut gateway = MarketDataGateway::new(bus.clone());

    // Valid event (no delay)
    let valid_event = MarketEvent {
        instrument_id: "NIFTY".into(),
        data_type: MarketDataType::Tick,
        timestamp: EventTimestamp {
            event_ts: 1000,
            receive_ts: 1000,
            source_ts: None,
        },
        sequence: None,
        source: "MOCK".into(),
        price: 25000.0,
        volume: None,
        bid: None,
        ask: None,
        open_interest: None,
        iv: None,
        delta: None,
        gamma: None,
    };
    assert!(gateway.validate_event(&valid_event).is_ok());

    // Stale event (>5000ms delay)
    let stale_event = MarketEvent {
        timestamp: EventTimestamp {
            event_ts: 1000,
            receive_ts: 6001,
            source_ts: None,
        },
        ..valid_event.clone()
    };
    assert_eq!(
        gateway.validate_event(&stale_event),
        Err("Stale data: event age exceeds 5 seconds")
    );

    // Malformed event (zero price)
    let malformed_event = MarketEvent {
        price: 0.0,
        ..valid_event.clone()
    };
    assert_eq!(
        gateway.validate_event(&malformed_event),
        Err("Malformed data: price <= 0")
    );
}

#[test]
fn test_gateway_duplicate_sequence() {
    let bus = Arc::new(EventBus::new());
    let mut gateway = MarketDataGateway::new(bus.clone());

    let mut ev = MarketEvent {
        instrument_id: "NIFTY".into(),
        data_type: MarketDataType::Tick,
        timestamp: EventTimestamp {
            event_ts: 1000,
            receive_ts: 1000,
            source_ts: None,
        },
        sequence: Some(10),
        source: "MOCK".into(),
        price: 25000.0,
        volume: None,
        bid: None,
        ask: None,
        open_interest: None,
        iv: None,
        delta: None,
        gamma: None,
    };

    assert!(gateway.validate_event(&ev).is_ok());

    // Send same sequence
    assert_eq!(
        gateway.validate_event(&ev),
        Err("Duplicate or out-of-order sequence detected")
    );

    // Send older sequence
    ev.sequence = Some(9);
    assert_eq!(
        gateway.validate_event(&ev),
        Err("Duplicate or out-of-order sequence detected")
    );

    // Send newer sequence
    ev.sequence = Some(11);
    assert!(gateway.validate_event(&ev).is_ok());
}

#[test]
fn test_subscription_manager_deduplication() {
    let mut mgr = SubscriptionManager::new();

    // First sub -> returns true (new sub)
    assert!(mgr.subscribe(
        "NIFTY_FUT",
        "StrategyA",
        SubscriptionTier::P1StrategyCandidates
    ));

    // Same sub, different strategy -> returns false (already subbed)
    assert!(!mgr.subscribe(
        "NIFTY_FUT",
        "StrategyB",
        SubscriptionTier::P1StrategyCandidates
    ));

    // Higher priority sub -> returns true (upgrade)
    assert!(mgr.subscribe(
        "NIFTY_FUT",
        "Portfolio",
        SubscriptionTier::P0ActivePositions
    ));

    let subs = mgr.get_active_subscriptions();
    assert_eq!(subs.len(), 1);

    // Remove one source -> returns false (still needed by others)
    assert!(!mgr.unsubscribe("NIFTY_FUT", "StrategyA"));
    // Remove second source -> returns false
    assert!(!mgr.unsubscribe("NIFTY_FUT", "StrategyB"));
    // Remove last source -> returns true (can unsubscribe from provider)
    assert!(mgr.unsubscribe("NIFTY_FUT", "Portfolio"));

    assert_eq!(mgr.get_active_subscriptions().len(), 0);
}
