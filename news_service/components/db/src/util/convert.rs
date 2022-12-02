use time::OffsetDateTime;

pub fn now_utc_datetime() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn now_utc_timestamp() -> i64 {
    now_utc_datetime().unix_timestamp()
}

pub fn datetime_from_unix_timestamp(timestamp: i64) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(timestamp).unwrap()
}

pub fn datetime_to_str(datetime: OffsetDateTime) -> String {
    datetime
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap()
}

/// Returns the number of seconds in an hour.
pub fn seconds_in_hour() -> i64 {
    60 * 60
}
