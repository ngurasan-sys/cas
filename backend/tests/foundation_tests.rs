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

#[test]
fn test_market_clock_sessions() {
    use backend::foundation::clock::SessionState;
    use chrono::{NaiveDate, NaiveDateTime, TimeZone};
    use chrono_tz::Asia::Kolkata;

    let clock = MarketClock::new();

    // Helper to create a specific datetime in Kolkata
    let mk_time = |y, m, d, h, min, s| {
        let naive = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(y, m, d).unwrap(),
            chrono::NaiveTime::from_hms_opt(h, min, s).unwrap(),
        );
        Kolkata.from_local_datetime(&naive).unwrap()
    };

    // A Wednesday (weekday)
    let pre_open = mk_time(2023, 10, 25, 9, 5, 0);
    assert_eq!(clock.session_state(pre_open, false), SessionState::PreOpen);

    let opening = mk_time(2023, 10, 25, 9, 10, 0);
    assert_eq!(clock.session_state(opening, false), SessionState::Opening);

    let normal = mk_time(2023, 10, 25, 12, 0, 0);
    assert_eq!(clock.session_state(normal, false), SessionState::Normal);

    let post_1515 = mk_time(2023, 10, 25, 15, 20, 0);
    assert_eq!(
        clock.session_state(post_1515, false),
        SessionState::Post1515
    );

    let pre_close = mk_time(2023, 10, 25, 15, 35, 0);
    assert_eq!(
        clock.session_state(pre_close, false),
        SessionState::PreClose
    );

    let closed = mk_time(2023, 10, 25, 16, 0, 0);
    assert_eq!(clock.session_state(closed, false), SessionState::Closed);

    let closed_morning = mk_time(2023, 10, 25, 8, 0, 0);
    assert_eq!(
        clock.session_state(closed_morning, false),
        SessionState::Closed
    );

    // A Saturday
    let weekend = mk_time(2023, 10, 28, 12, 0, 0);
    assert_eq!(clock.session_state(weekend, false), SessionState::Weekend);

    // Holiday flag overrides time
    assert_eq!(clock.session_state(normal, true), SessionState::Holiday);
}
