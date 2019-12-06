/*
  create at 2019/12/4 by 'itachy'
*/
use std::collections::HashMap;
use crate::convenience::{time::*,
                         tools::*};
use crate::pre_define::{const_define::account_info::*,
                        const_define::network::param_keys::*};
use crate::data_model::{api_code::ApiCodeRspData,
                        init_config::InitConfig};


pub(super) fn combine_get_api_code_uri(url: &str) -> String {
    url.to_string()
}

pub(super) fn combine_login_uri(url: &str, api_code_data: &ApiCodeRspData, login_config: &InitConfig) -> String {
    let ref mut params = HashMap::new();
    params.insert(K_APP_PASSWORD, login_config.password());
    params.insert(K_APP_PHONE_NUMBER, login_config.phone_number());
    params.insert(K_SIGNATURE, api_code_data.signature());
    let ref nonce = api_code_data.nonce().to_string();
    params.insert(K_NONCE, nonce);
    let ref timestamp = api_code_data.timestamp().to_string();
    params.insert(K_TIMESTAMP, timestamp);
    format!("{}?{}", url, flat_to_url_query_param(params))
}

pub(super) fn combine_order_uri(url: &str, api_code_data: &ApiCodeRspData) -> String {
    /*
    curl -X GET -H "User-Agent: PCDriving/2.25 (iPhone; iOS 11.1.2; Scale/2.00)" -H "Host: appwebsrv01.22168168.com" "http://appwebsrv01.22168168.com/PCMIS_App.asmx/AddCarOrderInfoHous_Ios?
    AppRecordID=E02D026E5E2E4DC48D81C22C5D55588D
    &ReservedID=5C176F61BACF44D88939EF64EEDC3D77
    &nonce=1259841888
    &signature=9f30a2eee14fbd954d415e4bb276b80e912aaf61
    &timestamp=1259841688"
    */
    let ref mut params = HashMap::new();
    params.insert(K_APP_RECORD_ID, APP_RECORD_ID);
    let ref reserved_id = reserved_id();
    params.insert(K_RESERVED_ID, reserved_id);
    params.insert(K_SIGNATURE, api_code_data.signature());
    let ref nonce = api_code_data.nonce().to_string();
    params.insert(K_NONCE, nonce);
    let ref timestamp = api_code_data.timestamp().to_string();
    params.insert(K_TIMESTAMP, timestamp);
    format!("{}?{}", url, flat_to_url_query_param(params))
}
