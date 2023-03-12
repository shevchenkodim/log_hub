use log::{info};
use crate::application::utils::date_times::utc_now;

pub fn console_log(srv: String, key: String, msg: String) {
    info!("[{}] Service: {}, Key: {}, Info: {}", utc_now(), srv, key, msg);
}
