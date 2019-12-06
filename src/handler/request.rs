/*
  create at 2019/12/6 by 'itachy'
*/
use crate::convenience::{errors, tools};
use crate::network::basic;
use crate::data_model::{ResponseWrap,
                        api_code::*,
                        login::*,
                        init_config::*};
use crate::pre_define::api_define::*;
use super::request_combine;


pub(super) fn request_signature(client: &reqwest::Client) -> Option<ApiCodeRspData> {
    match request_api_code(&client) {
        Err(err) => {
            error!("Request request_signature failed: {}", err);
            None
        },
        Ok(api_code) => Some(api_code.data()),
    }
}

fn request_api_code(client: &reqwest::Client) -> errors::Result<ApiCodeRsp> {
    let ref url = request_combine::combine_get_api_code_uri(GET_API_CODE);
    debug!("Request api_code use url: {}", url);
    let mut rsp = client.get(url).send()?;
    let ref rsp_content = rsp.text()?;
    debug!("Response<request_api_code> body text: '{}'", rsp_content);
    let unwrap_rsp = ResponseWrap::from(rsp_content)?;
    Ok(unwrap_rsp.body()?)
}


fn pure_login(client: &reqwest::Client, api_code_data: &ApiCodeRspData, login_config: &InitConfig) -> errors::Result<bool> {
    let ref url = request_combine::combine_login_uri(GET_LOGIN, &api_code_data, login_config);
    debug!("Request login use url: {}", url);
    let mut rsp = client.get(url).send()?;
    let ref rsp_content = rsp.text()?;
    debug!("Response<pure_login> body text: '{}'", rsp_content);
    let unwrap_rsp = ResponseWrap::from(rsp_content)?;
    let ref login: Login = unwrap_rsp.body()?;
    Ok(login.is_login_succeed())
}

pub(super) fn login(client: &reqwest::Client, login_config: &InitConfig) -> bool {
    match request_signature(client) {
        None => {
            error!("request_signature is None");
            false
        },
        Some(ref api_code_data) => {
            match pure_login(client, &api_code_data, login_config) {
                Err(err) => {
                    error!("login failed");
                    false
                },
                Ok(res) => res,
            }
        }
    }
}


//// 获取最新的预约列表 TODO: before running, Check Config is than chosen exist
//// return will_chosen_ReservedID
//pub(super) fn fetch_newest_reserved_time_list(client: &reqwest::Client) ->  {
//
//}
//
/////
//pub(super) fn apply_newest_car_oder(sprint_sec: u32) {
//
//}