use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;

// Hacker News formula:
// Score = (P-1) / (T+2)^G

// where,
// P = points of an item (and -1 is to negate submitters vote)
// T = time since submission (in hours)
// G = Gravity, defaults to 1.8 in news.arc

// Climate News version:
//
// url_score / (( hours_since_first_created +2 )^gravity)
//
pub fn time_decayed_url_score(
    gravity: Decimal,
    url_score: i32,
    hours_since_first_created: i64,
) -> i32 {
    let hour_addition = dec!(2);
    let url_score: Decimal = url_score.into();
    let hours_since_first_created: Decimal = hours_since_first_created.into();
    let time_value = hours_since_first_created
        .checked_add(hour_addition)
        .unwrap();

    let numerator: Decimal = url_score;
    let denominator: Decimal = time_value.checked_powd(gravity).unwrap();

    let time_decayed_url_score = numerator.checked_div(denominator).unwrap();
    time_decayed_url_score.to_i32().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn time_decayed_url_score_test_1() {
        let gravity = dec!(0.4);
        let url_score = 600;
        let hours_since_first_created = 5;
        let time_decayed_url_score =
            time_decayed_url_score(gravity, url_score, hours_since_first_created);
        assert_eq!(time_decayed_url_score, 226);
    }

    #[test]
    fn time_decayed_url_score_test_2() {
        let gravity = dec!(0.4);
        let url_score = 600;
        let hours_since_first_created = 24;
        let time_decayed_url_score =
            time_decayed_url_score(gravity, url_score, hours_since_first_created);
        assert_eq!(time_decayed_url_score, 117);
    }
}
