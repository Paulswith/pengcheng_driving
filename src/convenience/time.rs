/*
  create at 2019/12/4 by 'itachy'
*/
use chrono::{DateTime, Local, TimeZone, Datelike};


/// newest date is today + 2
pub fn get_newest_order_date() -> String {
    let today: DateTime<Local> = Local::now();
    let newest_order_date = today + time::Duration::days(2);
    newest_order_date.format("%Y-%m-%d").to_string()
}

/// timestamp for now
pub fn timestamp_for_now() -> i64 {
    let now: DateTime<Local> = Local::now();
    now.timestamp()
}

/// param: at_hour '14:00:00'
pub fn timestamp_from_today(at_hour: u32) -> i64 {
    let now: DateTime<Local> = Local::now();
    let special_date: DateTime<Local> = Local
        .ymd(now.year(), now.month(), now.day())
        .and_hms(at_hour, 0, 0);
    special_date.timestamp()
}
