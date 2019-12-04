/*
  create at 2019/12/4 by 'itachy'
*/
use std::collections::HashMap;
use crate::convenience::{time::*,
                         tools::*};
use crate::pre_define::{const_define::account_info::*,
                        const_define::network::param_keys::*};


pub(super) fn combine_get_api_code_uri(url: &str) -> String {
    url.to_string()
}

pub(super) fn combine_login_uri(url: &str, signature: &str) -> String {
    let ref mut params = HashMap::new();
    params.insert(K_APP_PASSWORD, APP_PASSWORD);
    params.insert(K_APP_PHONE_NUMBER, APP_PHONE_NUMBER);
    params.insert(K_SIGNATURE, signature);
    let ref nonce= generate_nonce();
    params.insert(K_NONCE, nonce);
    let ref timestamp = cal_timestamp_at_today().to_string();
    params.insert(K_TIMESTAMP, timestamp);
    format!("{}?{}", url, flat_to_url_query_param(params))
}
