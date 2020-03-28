use chrono::{DateTime, Utc, Duration};
use std::ops::Add;

const ONE_GIGASECOUND_IN_SECONDS: i64 = 1000000000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(ONE_GIGASECOUND_IN_SECONDS)
}
