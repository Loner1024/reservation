use chrono::{DateTime, NaiveDateTime, Utc};
use prost_types::Timestamp;

pub use pb::*;

mod pb;

pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as u32).unwrap(),
        Utc,
    )
}
