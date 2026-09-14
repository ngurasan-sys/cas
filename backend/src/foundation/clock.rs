use chrono::{DateTime, NaiveTime, Utc};
use chrono_tz::Asia::Kolkata;
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SessionState {
    PreOpen,
    Opening,
    Normal,
    Post1515,
    PreClose,
    Closed,
    Holiday,
    Weekend,
}

pub struct MarketClock {
    pub timezone: Tz,
}

impl MarketClock {
    pub fn new() -> Self {
        Self { timezone: Kolkata }
    }

    /// Helper to get current time in Kolkata
    pub fn now(&self) -> DateTime<Tz> {
        Utc::now().with_timezone(&self.timezone)
    }

    /// Evaluates current session state based on time
    ///
    /// This method allows injecting a specific time to make it testable,
    /// and uses an optional flag `is_holiday` to test holiday boundaries.
    pub fn session_state(&self, now: DateTime<Tz>, is_holiday: bool) -> SessionState {
        if is_holiday {
            return SessionState::Holiday;
        }

        // Weekend check
        let weekday = now.format("%w").to_string();
        if weekday == "0" || weekday == "6" {
            return SessionState::Weekend;
        }

        let time = now.time();

        let pre_open_start = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
        let pre_open_end = NaiveTime::from_hms_opt(9, 8, 0).unwrap();
        let normal_start = NaiveTime::from_hms_opt(9, 15, 0).unwrap();
        let post_1515_start = NaiveTime::from_hms_opt(15, 15, 0).unwrap();
        let close_start = NaiveTime::from_hms_opt(15, 30, 0).unwrap();
        let post_close_start = NaiveTime::from_hms_opt(15, 40, 0).unwrap();

        if time >= pre_open_start && time < pre_open_end {
            SessionState::PreOpen
        } else if time >= pre_open_end && time < normal_start {
            SessionState::Opening
        } else if time >= normal_start && time < post_1515_start {
            SessionState::Normal
        } else if time >= post_1515_start && time < close_start {
            SessionState::Post1515
        } else if time >= close_start && time < post_close_start {
            SessionState::PreClose
        } else {
            SessionState::Closed
        }
    }

    pub fn current_session(&self) -> SessionState {
        // In reality, holiday logic would lookup a db or config here
        self.session_state(self.now(), false)
    }
}
