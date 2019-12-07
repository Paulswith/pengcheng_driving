/*
  create at 2019/12/6 by 'itachy'
*/
use crate::convenience::{errors, tools};
use crate::network::basic;
use crate::data_model::{ResponseWrap,
                        api_code::*,
                        login::*,
                        init_config::*,
                        reserved::*,
                        teach_info::*};
use crate::pre_define::api_define::*;
use crate::convenience::traits::*;
use super::uri_combine;


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
    let ref url = uri_combine::combine_get_api_code_uri(GET_API_CODE);
    debug!("Request api_code use url: {}", url);
    let mut rsp = client.get(url).send()?;
    let ref rsp_content = rsp.text()?;
    debug!("Response<request_api_code> body text: '{}'", rsp_content);
    let unwrap_rsp = ResponseWrap::from(rsp_content)?;
    Ok(unwrap_rsp.body()?)
}


fn pure_login(client: &reqwest::Client,
              api_code_data: &ApiCodeRspData,
              config: &Config) -> errors::Result<bool> {
    let ref url = uri_combine::combine_login_uri(GET_LOGIN,
                                                 &api_code_data,
                                                 config);
    debug!("Request login use url: {}", url);
    let mut rsp = client.get(url).send()?;
    let ref rsp_content = rsp.text()?;
    debug!("Response<pure_login> body text: '{}'", rsp_content);
    let unwrap_rsp = ResponseWrap::from(rsp_content)?;
    let ref login: Login = unwrap_rsp.body()?;
    Ok(login.is_req_succeed())
}

pub(super) fn login(client: &reqwest::Client, config: &Config) -> bool {
    match request_signature(client) {
        None => {
            error!("login, request_signature is None");
            false
        },
        Some(ref api_code_data) => {
            match pure_login(client, &api_code_data, config) {
                Err(err) => false,
                Ok(res) => res,
            }
        }
    }
}

///
fn pure_fetch_teach_info(client: &reqwest::Client,
                         api_code_data: &ApiCodeRspData,
                         config: &Config) -> errors::Result<TeachInfo> {
    let ref url = uri_combine::combine_teach_info(GET_TEACH_INFO,
                                                          &api_code_data,
                                                          config);
    debug!("Request fetch_teach_info use url: {}", url);
    let mut rsp = client.get(url).send()?;
    let ref rsp_content = rsp.text()?;
    let unwrap_rsp = ResponseWrap::from(rsp_content)?;
    Ok(
        unwrap_rsp.body()?
    )
}

///
pub(super) fn fetch_teach_info(client: &reqwest::Client, config: &Config) -> Option<TeachInfo> {
    match request_signature(client) {
        None => {
            error!("fetch_teach_info, request_signature is None");
            None
        },
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

///
fn pure_fetch_newest_reserved_time_list(client: &reqwest::Client,
                                        api_code_data: &ApiCodeRspData,
                                        config: &Config,
                                        teach_info: &TeachInfo) -> errors::Result<Reserved> {
    let ref url = uri_combine::combine_reserved_time_list(GET_RESERVED_TIME_LIST,
                                                          &api_code_data,
                                                          config,
                                                          teach_info);
    debug!("Request fetch_newest_reserved_time_list use url: {}", url);
    let mut rsp = client.get(url).send()?;
    let ref rsp_content = rsp.text()?;
    debug!("Response<pure_fetch_newest_reserved_time_list> body text: '{}'", rsp_content);
    let unwrap_rsp = ResponseWrap::from(rsp_content)?;
    Ok(
        unwrap_rsp.body()?
    )
}


/// 获取最新的预约列表
// return will_chosen_ReservedID
pub(super) fn fetch_newest_reserved_time_list(client: &reqwest::Client,
                                              config: &Config,
                                              teach_info: &TeachInfo) -> Option<Reserved> {
    match request_signature(client) {
        None => {
            error!("fetch_newest_reserved_time_list, request_signature is None");
            None
        },
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

///
pub(super) fn apply_car_order(client: &reqwest::Client,
                              config: &Config,
                              reserved: &ReservedTime) {

}