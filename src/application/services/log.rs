// use chrono::{DateTime, Utc};
// use crypto::sha3::Sha3;
// use serde_json::Value;
// use crate::{PostgresClient, UserEntity};
// use crate::adapters::spi::db::repositories::logger_repository::LoggerRepository;
// use crate::application::utils::date_times::utc_now;
//
// use crate::domain::logger_entity::{LoggerEntity};
// use crate::domain::logger_event_type_entity::LoggerEventTypeEntity;
// use crate::domain::logger_level_entity::LoggerLevelEntity;
//
// pub struct LoggerService {
//     repository: LoggerRepository,
// }
//
// impl LoggerService {
//     pub fn new(repository: LoggerRepository) -> Self {
//         LoggerService {
//             repository
//         }
//     }
// }
//
// impl LoggerService {
//     // pub async fn create(&self, db: &PostgresClient, mut log_entity: LogEntity) -> Result<bool, String> {
//     //     log_entity.created_at = Some(utc_now());
//     //     if log_entity.logger_name.len() == 0 {
//     //         log_entity.logger_name = "default".to_string()
//     //     }
//     //     self.repository.create(&db, log_entity).await
//     // }
//     // pub async fn get_by_filter(&self, db: &PostgresClient, limit: i64, offset: i64, level: Option<i64>, event: Option<i64>, source: Option<i64>, logger: Option<i64>, search: Option<&str>, d_start: Option<DateTime<Utc>>, d_end: Option<DateTime<Utc>>) -> Result<(Vec<LoggerEntity>, i64), String> {
//     //     self.repository.get_by_filter(db, limit, offset, level, event, source, logger, search, d_start, d_end).await
//     // }
// }