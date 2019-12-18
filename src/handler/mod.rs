/*
  create at 2019/12/4 by 'itachy'
*/
mod request;
mod uri_combine;
mod pure_request;
use crate::convenience::time;
use crate::network::basic;
use crate::data_model::{init_config::*, teach_info::TeachInfo};
use crate::pre_define::const_define::timer::*;
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

///  逐步请求, 模拟用户登录的过程.
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
    let target_timestamp = time::timestamp_from_today(config.hour_of_ticket_rush_time());
    let beforehand_timestamp = target_timestamp - (config.beforehand_order_second() as i64);
    // 4.2 loop, until current timestamp >= tx
    info!("Found target time: {}", config.order_start_time());
    // 心跳:
    head_beat_keep_alive(client, config, teach_info, beforehand_timestamp);
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

/// 心跳保持在线状态至准备抢票阶段.
fn head_beat_keep_alive(client: &reqwest::Client,
                        config: &Config,
                        teach_info: &TeachInfo,
                        beforehand_timestamp: i64) {
    info!("Into heart beat state...");
    let target_time_approach = beforehand_timestamp - (config.heart_beat_interval() as i64);
    debug!("target_time_approach: {}", target_time_approach);
    loop {
        // 避免sleep后超时, 绕前判断
        if time::timestamp_for_now() + AVOID_TIMEOUT_FOR_HEART >= target_time_approach {
            info!("Heart beat done.");
            break;
        }
        let reserved = request::fetch_newest_reserved_time_list(client, config, teach_info);
        if reserved.is_none(){
            error!("Keep alive fail");
        }
        sleep(Duration::from_secs(config.heart_beat_interval() as u64));
    }
}

/// sprint request
fn sprint_request(client: &reqwest::Client, config: &Config, reserved_record_id: &str) {
    // 提前N秒 + 跨越2秒. 每秒发起请求4次.
    let total_send_count = (config.beforehand_order_second() + SEND_REQUEST_BEFORE_SECONDS) * SEND_REQUEST_PER_SECOND;
    let sleep_wait = 1.0 / (SEND_REQUEST_PER_SECOND as f32);
    debug!("Calculate sleep for wait per second: {}", sleep_wait);
    for _ in 0..total_send_count {
        debug!("Send request");
        if request::apply_car_order(client, config, reserved_record_id) {
            info!("Apply order succeed!");
            return;
        }
        sleep(Duration::from_secs_f32(sleep_wait));
    }
    info!("Apply order unsuccessfully!");
}
