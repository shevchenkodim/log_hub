use chrono::{DateTime, Duration, TimeZone, Utc};


pub fn utc_now() -> DateTime<Utc> {
    Utc::now()
}


pub fn utc_after_seconds(s: i64) -> DateTime<Utc> {
    utc_now() + Duration::seconds(s)
}


pub fn utc_before_seconds(s: i64) -> DateTime<Utc> {
    utc_now() - Duration::seconds(s)
}


pub fn utc_after_hours(h: i64) -> DateTime<Utc> {
    utc_now() + Duration::hours(h)
}


pub fn utc_before_hours(h: i64) -> DateTime<Utc> {
    utc_now() - Duration::hours(h)
}


pub fn utc_after_days(d: i64) -> DateTime<Utc> {
    utc_now() + Duration::days(d)
}


pub fn utc_before_days(d: i64) -> DateTime<Utc> {
    utc_now() - Duration::days(d)
}


pub fn parse_utc_date_from_str(d: &str) -> (bool, Option<DateTime<Utc>>) {
    match Utc::datetime_from_str(&Utc::now().timezone(), &*format!("{d} 00:00:00"), "%Y-%m-%d %H:%M:%S") {
        Ok(t) => (true, Some(DateTime::from(t))),
        Err(_) => (false, None)
    }
}
