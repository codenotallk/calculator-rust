use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{NaiveDateTime, TimeZone, Utc};

use super::verfication::is_date_valid;

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

pub fn get_epoch_from_formatted(date: &String) -> Result<i64, &'static str> {
    if is_date_valid(&date) {
        Ok(
            NaiveDateTime::parse_from_str(date.as_str(), "%d/%m/%Y %H:%M:%S")
                .unwrap()
                .and_utc()
                .timestamp(),
        )
    } else {
        Err("Not a valid date")
    }
}
