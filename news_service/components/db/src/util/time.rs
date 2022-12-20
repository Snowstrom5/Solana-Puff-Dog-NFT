use super::convert::datetime_from_unix_timestamp;
use super::convert::now_utc_datetime;
use anyhow::bail;
use anyhow::Result;
use time::ext::NumericalDuration;
use time::Date;
use time::Duration;
use time::Month;
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
    } else {
        // 90 days in release mode
        now_utc_timestamp().checked_add((-90).days()).unwrap()
    }
}

pub fn past_days(days: i64) -> OffsetDateTime {
    now_utc_timestamp().checked_add((-days).days()).unwrap()
}

fn now_utc_timestamp() -> OffsetDateTime {
    let now_utc_timestamp = now_utc_datetime();
    // Remove nanoseconds to avoid Twitter API error
    now_utc_timestamp
        .checked_add(-time::Duration::nanoseconds(
            now_utc_timestamp.nanosecond() as i64
        ))
        .unwrap()
}

pub fn timestamp_from_month_year(month: i32, year: i32) -> Result<i64> {
    let month: Month = month_from_number(month)?;
    let date = Date::from_calendar_date(year, month, 1)?;
    let time = date.with_hms(0, 0, 0).unwrap();
    let unix_timestamp = time.assume_utc().unix_timestamp();
    Ok(unix_timestamp)
}

fn month_from_number(n: i32) -> Result<Month, anyhow::Error> {
    match n {
        1 => Ok(Month::January),
        2 => Ok(Month::February),
        3 => Ok(Month::March),
        4 => Ok(Month::April),
        5 => Ok(Month::May),
        6 => Ok(Month::June),
        7 => Ok(Month::July),
        8 => Ok(Month::August),
        9 => Ok(Month::September),
        10 => Ok(Month::October),
        11 => Ok(Month::November),
        12 => Ok(Month::December),
        _ => bail!("invalid month_from_number"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_timestamp_from_month_year_1() {
        let expected_date = datetime!(2020 - 01 - 01  0:00)
            .assume_utc()
            .unix_timestamp();
        let date = timestamp_from_month_year(1, 2020).unwrap();
        assert_eq!(date, expected_date);
    }

    #[test]
    fn test_timestamp_from_month_year_2() {
        let expected_date = datetime!(2022 - 06 - 01  0:00)
            .assume_utc()
            .unix_timestamp();
        let date = timestamp_from_month_year(6, 2022).unwrap();
        assert_eq!(date, expected_date);
    }
}
