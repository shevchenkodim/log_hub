use chrono::{DateTime, Utc};
use serde_json::{Value};
use crate::domain::user_entity::UserEntity;


#[derive(Debug, Clone)]
pub struct LogEntity {
    pub id: Option<i32>,
    pub level: i32,
    pub event: i32,
    pub source: String,
    pub logger_name: String,
    pub user: Option<UserEntity>,
    pub msg: String,
    pub params: Value,
    pub created_at: Option<DateTime<Utc>>
}
