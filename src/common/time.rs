use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{TimeZone, Utc};

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn date_formatted(date: i64) -> String {
    Utc.timestamp_opt(date, 0)
        .unwrap()
        .format("%d/%m/%Y %H:%M:%S")
        .to_string()
}
