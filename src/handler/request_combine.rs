/*
  create at 2019/12/4 by 'itachy'
*/
use std::collections::HashMap;
use crate::convenience::{time::*,
                         tools::*};
use crate::pre_define::{const_define::account_info::*,
                        const_define::network::param_keys::*};
use crate::data_model::api_code::ApiCodeRspData;


pub(super) fn combine_get_api_code_uri(url: &str) -> String {
    url.to_string()
}

pub(super) fn combine_login_uri(url: &str, api_code_data: &ApiCodeRspData) -> String {
    let ref mut params = HashMap::new();
    params.insert(K_APP_PASSWORD, APP_PASSWORD);
    params.insert(K_APP_PHONE_NUMBER, APP_PHONE_NUMBER);
    params.insert(K_SIGNATURE, api_code_data.signature());
//    let ref nonce= generate_nonce();

//    let ref timestamp = cal_timestamp_at_today().to_string();
//    let ref timestamp = cur_timestamp();
    let ref nonce = api_code_data.nonce().to_string();
    params.insert(K_NONCE, nonce);
    let ref timestamp = api_code_data.timestamp().to_string();
    params.insert(K_TIMESTAMP, timestamp);
    format!("{}?{}", url, flat_to_url_query_param(params))
}
