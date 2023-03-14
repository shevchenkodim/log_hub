// use crypto::digest::Digest;
// use crypto::sha3::Sha3;
//
// use crate::application::utils::date_times::utc_now;
// use crate::{PostgresClient, UserEntity};
// use crate::adapters::spi::db::repositories::user_repository::UserRepository;
// use crate::application::services::user_cache::UserCacheService;
//
// pub struct UserService {
//     repository: UserRepository,
// }
//
// impl UserService {
//     pub fn new(repository: UserRepository) -> Self {
//         UserService { repository }
//     }
// }
//
// impl UserService {
//     pub async fn create(&self, db: &PostgresClient, user: UserEntity) -> Result<bool, String> {
//         // prepare user
//         let mut user: UserEntity = user.clone();
//         user.password = self.encode_password(user.password);
//         user.last_login = Some(utc_now());
//
//         let success = self.repository.create(db, &user).await;
//         match success {
//             Ok(_) => Ok(true),
//             Err(err) => Err(err.to_string())
//         }
//     }
//     pub async fn check_access(&self, db: &PostgresClient, username: String, password: String) -> (bool, Option<i32>) {
//         match self.repository.get_by_username(db, username).await {
//             Ok(user) => {
//                 match user.check_password(self.encode_password(password)) {
//                     true => {
//                         match user.id {
//                             Some(pk) => (true, Some(pk)),
//                             _ => (false, None)
//                         }
//                     },
//                     false => (false, None)
//                 }
//             },
//             Err(_) => (false, None)
//         }
//     }
//     pub async fn get_by_pk(&self, db: &PostgresClient, pk: i32) -> Result<UserEntity, String> {
//         self.repository.get_by_pk(db, pk).await
//     }
//     fn encode_password(&self, password: String) -> String {
//         let mut hasher = Sha3::sha3_256();
//         hasher.input_str(&password);
//         hasher.result_str()
//     }
// }
