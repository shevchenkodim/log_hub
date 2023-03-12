use chrono::{DateTime, Utc};


#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub last_login: Option<DateTime<Utc>>
}
