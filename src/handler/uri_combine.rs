/*
  create at 2019/12/4 by 'itachy'
*/
use std::collections::HashMap;
use crate::convenience::{time::*,
                         tools::*};
use crate::pre_define::const_define::network::param_keys::*;
use crate::data_model::{api_code::ApiCodeRspData,
                        init_config::Config,
                        teach_info::TeachInfo};


pub(super) fn combine_get_api_code_uri(url: &str) -> String {
    url.to_string()
}


pub(super) fn combine_login_uri(url: &str, api_code_data: &ApiCodeRspData, config: &Config) -> String {
    let ref mut params = HashMap::new();
    let ref nonce = api_code_data.nonce().to_string();
    let ref timestamp = api_code_data.timestamp().to_string();
    params.insert(K_APP_PASSWORD, config.password());
    params.insert(K_APP_PHONE_NUMBER, config.phone_number());
    params.insert(K_SIGNATURE, api_code_data.signature());
    params.insert(K_NONCE, nonce);
    params.insert(K_TIMESTAMP, timestamp);
    format!("{}?{}", url, flat_to_url_query_param(params))
}

///
pub(super) fn combine_teach_info(url: &str, api_code_data: &ApiCodeRspData, config: &Config) -> String {
    let ref mut params = HashMap::new();
    let ref nonce = api_code_data.nonce().to_string();
    let ref timestamp = api_code_data.timestamp().to_string();
    params.insert(K_APP_RECORD_ID, config.app_record_id());
    params.insert(K_NONCE, nonce);
    params.insert(K_SIGNATURE, api_code_data.signature());
    params.insert(K_TIMESTAMP, timestamp);
    format!("{}?{}", url, flat_to_url_query_param(params))
}

/// orderable list uri
pub(super) fn combine_reserved_time_list(url: &str,
                                         api_code_data: &ApiCodeRspData,
                                         config: &Config,
                                         teach_info: &TeachInfo) -> String {
    let ref mut params = HashMap::new();
    let ref teach_id = teach_info.teach_no();
    let ref phase = config.phase().to_string();
    let ref reserved_date = get_newest_order_date();
    let ref nonce = api_code_data.nonce().to_string();
    let ref timestamp = api_code_data.timestamp().to_string();
    params.insert(K_APP_RECORD_ID, config.app_record_id());
    params.insert(K_PHASE, phase);
    params.insert(K_TEACH_ID, teach_id);
    params.insert(K_RESERVED_DATE, reserved_date);
    params.insert(K_SIGNATURE, api_code_data.signature());
    params.insert(K_NONCE, nonce);
    params.insert(K_TIMESTAMP, timestamp);
    format!("{}?{}", url, flat_to_url_query_param(params))
}

///// order uri
//pub(super) fn combine_order_uri(url: &str, api_code_data: &ApiCodeRspData) -> String {
//    let ref mut params = HashMap::new();
//    let ref reserved_id = reserved_id();
//    let ref nonce = api_code_data.nonce().to_string();
//    let ref timestamp = api_code_data.timestamp().to_string();
//    params.insert(K_APP_RECORD_ID, APP_RECORD_ID);
//    params.insert(K_RESERVED_ID, reserved_id);
//    params.insert(K_SIGNATURE, api_code_data.signature());
//    params.insert(K_NONCE, nonce);
//    params.insert(K_TIMESTAMP, timestamp);
//    format!("{}?{}", url, flat_to_url_query_param(params))
//}
