use backend::foundation::clock::MarketClock;
use backend::foundation::types::CorrelationId;

#[test]
fn test_correlation_id() {
    let id1 = CorrelationId::new();
    let id2 = CorrelationId::new();
    assert_ne!(id1.0, id2.0);
}

#[test]
fn test_market_clock_init() {
    let clock = MarketClock::new();
    let now = clock.now();
    assert_eq!(now.timezone().name(), "Asia/Kolkata");
}
