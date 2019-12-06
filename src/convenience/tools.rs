/*
create at 2019/12/4 by itachy
*/
use std::collections::HashMap;
use rand::Rng;
use super::errors;
use chrono::{DateTime, Local};


/// nonce
pub fn generate_nonce() -> String {
    let mut t = rand::thread_rng();
    format!("{}", t.gen_range(100_000_000, 1_000_000_000 - 1))
}

/// flat multi params to 'a=1&b=2'
pub fn flat_to_url_query_param(params: &HashMap<&str, &str>) -> String {
    let flat_param_vec: Vec<_> = params.into_iter()
        .map(|(k, v)|
            format!("{}={}", k, v))
        .collect();
    flat_param_vec.join("&")
}

/// newest date is today + 2
pub fn get_newest_order_date() -> String {
    let today: DateTime<Local> = Local::now();
    let newest_order_date = today + time::Duration::days(2);
    newest_order_date.format("%Y-%m-%d").to_string()
}