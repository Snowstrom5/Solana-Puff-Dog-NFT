use twitter_v2::{data::ReferencedTweetKind, id::NumericId};

pub fn opt_numeric_id_to_opt_i64(opt_numeric: Option<NumericId>) -> Option<i64> {
    opt_numeric.map_or_else(|| None, |numeric| Some(numeric_id_to_i64(numeric)))
}

pub fn numeric_id_to_i64(numeric: NumericId) -> i64 {
    numeric.as_u64() as i64
}

pub fn opt_i64_to_opt_numeric_id(opt_i: Option<i64>) -> Option<NumericId> {
    opt_i.map_or_else(|| None, |i| Some(i64_to_numeric_id(i)))
}

pub fn i64_to_numeric_id(i: i64) -> NumericId {
    NumericId::new(i as u64)
}

pub fn referenced_tweet_kind_to_string(referenced_tweet_kind: ReferencedTweetKind) -> String {
    match referenced_tweet_kind {
        ReferencedTweetKind::Quoted => "quoted",
        ReferencedTweetKind::RepliedTo => "replied_to",
        ReferencedTweetKind::Retweeted => "retweeted",
    }
    .to_string()
}
