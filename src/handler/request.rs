/*
  create at 2019/12/6 by 'itachy'
*/
use crate::data_model::init_config::*;
use crate::data_model::{reserved::*,
                        teach_info::*};
use super::pure_request::*;
use crate::convenience::traits::*;


/// 登录, 依赖登录的用户名密码
pub(super) fn login(client: &reqwest::Client, config: &Config) -> bool {
    match request_signature(client) {
        None => {
            error!("login, request_signature is None");
            false
        }
        Some(ref api_code_data) => {
            match pure_login(client, api_code_data, config) {
                Err(err) => {
                    error!("Login error: {}", err);
                    false
                }
                Ok(res) => res,
            }
        }
    }
}

/// 获取预约列表需要教练号, 返回教练结构体
pub(super) fn fetch_teach_info(client: &reqwest::Client, config: &Config) -> Option<TeachInfo> {
    match request_signature(client) {
        None => {
            error!("fetch_teach_info, request_signature is None");
            None
        }
        Some(ref api_code_data) => {
            match pure_fetch_teach_info(client, api_code_data, config) {
                Err(err) => {
                    error!("Fetch teach_info failed: {}", err);
                    None
                }
                Ok(reserved) => Some(reserved),
            }
        }
    }
}


/// 获取最新的预约列表
pub(super) fn fetch_newest_reserved_time_list(client: &reqwest::Client,
                                              config: &Config,
                                              teach_info: &TeachInfo) -> Option<Reserved> {
    match request_signature(client) {
        None => {
            error!("fetch_newest_reserved_time_list, request_signature is None");
            None
        }
        Some(ref api_code_data) => {
            match pure_fetch_newest_reserved_time_list(client, api_code_data, config, teach_info) {
                Err(err) => {
                    error!("Fetch newest_reserved_time_list failed: {}", err);
                    None
                }
                Ok(reserved) => Some(reserved),
            }
        }
    }
}

/// 匹配时间后, 根据想要预约时间字符串匹配其reserved_id, 进行预约请求
pub(super) fn apply_car_order(client: &reqwest::Client,
                              config: &Config,
                              reserved_id: &str) -> bool {
    match request_signature(client) {
        None => {
            error!("apply_car_order, request_signature is None");
            false
        }
        Some(ref api_code_data) => {
            match pure_apply_car_order(client, api_code_data, config, reserved_id) {
                Err(err) => {
                    error!("Fetch apply_car_order failed: {}", err);
                    false
                }
                Ok(ref order_info) => {
                    if order_info.is_req_succeed() {
                        info!("Response msg: {}", order_info.message());
                        true
                    } else {
                        debug!("Apply_car_order, response msg: {}", order_info.message());
                        false
                    }
                }
            }
        }
    }
}
