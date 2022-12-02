use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Referenced Twitter User data
#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct NewsTwitterReferencedUser {
    pub user_id: i64,
    pub username: String,
}
