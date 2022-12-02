use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Twitter list
#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsTwitterList {
    pub list_id: i64,
    pub last_checked_at: i64,
}
