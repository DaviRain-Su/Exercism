use chrono::{DateTime, Utc};
use time::Duration;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    unimplemented!("What time is a gigasecond later than {}", start);
    // start + Duration::seconds(1_000_000_000)
}
