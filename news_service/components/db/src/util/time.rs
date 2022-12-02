use super::convert::datetime_from_unix_timestamp;
use super::convert::now_utc_datetime;
use time::ext::NumericalDuration;
use time::Duration;
use time::OffsetDateTime;

pub fn datetime_minutes_diff(date_timestamp: i64) -> i64 {
    let date = datetime_from_unix_timestamp(date_timestamp);
    let diff: Duration = now_utc_datetime() - date;
    diff.whole_minutes()
}

pub fn datetime_hours_diff(date_timestamp: i64) -> i64 {
    let date = datetime_from_unix_timestamp(date_timestamp);
    let diff: Duration = now_utc_datetime() - date;
    diff.whole_hours()
}

pub fn lookup_period() -> OffsetDateTime {
    if cfg!(debug_assertions) {
        // 3 days in debug mode
        now_utc_timestamp().checked_add((-3).days()).unwrap()
    }else{
        // 90 days in release mode
        now_utc_timestamp().checked_add((-90).days()).unwrap()
    }
}

pub fn past_days(days: i64) -> OffsetDateTime {
    now_utc_timestamp().checked_add((-days).days()).unwrap()
}

fn now_utc_timestamp() -> OffsetDateTime {
    let now_utc_timestamp = now_utc_datetime();
    // Removes nanoseconds to avoid Twitter API error
    now_utc_timestamp
        .checked_add(-time::Duration::nanoseconds(
            now_utc_timestamp.nanosecond() as i64
        ))
        .unwrap()
}
