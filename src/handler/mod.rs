/*
  create at 2019/12/4 by 'itachy'
*/
mod request;
mod uri_combine;
use crate::convenience::time;
use crate::network::basic;
use crate::data_model::init_config::*;
use std::{thread::sleep,
          time::Duration};


pub fn handle_entry(config: &Config) {
    match basic::construct_client() {
        Err(err) => error!("Construct client failed: {}", err),
        Ok(ref client) => {
            handle_by_step(client, config);
        }
    }
}

///
fn handle_by_step(client: &reqwest::Client, config: &Config) {
    info!("##################Step 1. login##################");
    if !request::login(client, config) {
        error!("Login failed");
        return;
    }
    info!("Login succeed.");

    info!("##################Step 2. fetch teach_id##################");
    let teach_info = request::fetch_teach_info(client, config);
    if teach_info.is_none() {
        error!("Fetch teach_info failed.");
        return;
    }
    info!("Fetch teach_info succeed.");

    info!("##################Step 3. fetch reserved list##################");
    let ref teach_info = teach_info.unwrap();
    let reserved = request::fetch_newest_reserved_time_list(client, config, teach_info);
    if reserved.is_none(){
        error!("Fetch reserved failed.");
        return;
    }
    info!("Fetch newest_reserved_time_list succeed.");

    info!("##################Step 4. matching reserved id##################");
    let ref hour_of_order_time = config.order_start_time();
    let target_reserved_id = reserved.unwrap().find_valid_reserved_id(hour_of_order_time);
    if target_reserved_id.is_none() {
        error!("Not found special time record_id at {}", hour_of_order_time);
        return;
    }
    info!("Found target_reserved_id={:?} at {}", target_reserved_id,  hour_of_order_time);

    info!("##################Step 5. monitoring to sprint request##################");
    /*
        var_a=today.order_start_time as timestamp,
        sprint time beforehand_timestamp=$(var_a - config.beforehand_order_second) as timestamp
    */
    let target_timestamp = time::timestamp_from_today(config.hour_of_order_start_time());
    let beforehand_timestamp = target_timestamp - (config.beforehand_order_second() as i64);
    // 4.2 loop, until current timestamp >= tx
    info!("Found target time: {}", config.order_start_time());
    info!("Looping,  target_timestamp={} until archive sprint timestamp: {}", target_timestamp, beforehand_timestamp);
    loop {
        if time::timestamp_for_now() >= beforehand_timestamp {
            info!("Start sprint request");
            sprint_request(client, config, &target_reserved_id.unwrap());
            info!("Sprint request finished");
            break;
        }
        sleep(Duration::from_secs(1));
    }
}

/// sprint request
fn sprint_request(client: &reqwest::Client, config: &Config, reserved_record_id: &str) {
    let org_send_count = config.beforehand_order_second() * 5;
    let send_count = org_send_count + 1;
    let sleep_wait = (config.beforehand_order_second() as f32) / (org_send_count as f32);
    // （beforehand_order_second / 0.2 => beforehand_order_second * 5） + 1，间隔0.2秒发送一次请求
    for _ in 0..send_count {
        debug!("Send request");
        let res = request::apply_car_order(client, config, reserved_record_id);
        if res {
            info!("Apply order succeed!");
            return;
        }
        sleep(Duration::from_secs_f32(sleep_wait));
    }
    info!("Apply order unsuccessfully!");
}
