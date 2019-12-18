/*
  create at 2019/12/18 by 'itachy'
*/
use crate::convenience::errors;
use crate::data_model::{ResponseWrap,
                        api_code::*,
                        login::*,
                        init_config::*,
                        reserved::*,
                        teach_info::*,
                        order_info::*};
use crate::pre_define::api_define::*;
use super::uri_combine;
use crate::convenience::traits::*;


/// 仅内部接口 提供签名解析
fn request_api_code(client: &reqwest::Client) -> errors::Result<ApiCodeRsp> {
    let ref url = uri_combine::combine_get_api_code_uri(GET_API_CODE);
    debug!("Request api_code use url: {}", url);
    let mut rsp = client.get(url).send()?;
    let ref rsp_content = rsp.text()?;
    debug!("Response<request_api_code> body text: '{}'", rsp_content);
    let unwrap_rsp = ResponseWrap::from(rsp_content)?;
    Ok(unwrap_rsp.body()?)
}

/// 每次进行接口请求, 都必须走这个接口获取一个签名
pub(super) fn request_signature(client: &reqwest::Client) -> Option<ApiCodeRspData> {
    match request_api_code(&client) {
        Err(err) => {
            error!("Request request_signature failed: {}", err);
            None
        }
        Ok(api_code) => Some(api_code.data()),
    }
}

/// 登录逻辑
pub(super) fn pure_login(client: &reqwest::Client,
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

///
pub(super) fn pure_fetch_teach_info(client: &reqwest::Client,
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
pub(super) fn pure_apply_car_order(client: &reqwest::Client,
                                   api_code_data: &ApiCodeRspData,
                                   config: &Config,
                                   reserved_id: &str) -> errors::Result<OrderInfo> {
    let ref url = uri_combine::combine_order_uri(ADD_CAR_ORDER_INFO_HOUS_IOS,
                                                 &api_code_data,
                                                 config,
                                                 reserved_id);
    debug!("Request apply_car_order use url: {}", url);
    let mut rsp = client.get(url).send()?;
    let ref rsp_content = rsp.text()?;
    debug!("Response<pure_apply_car_order> body text: '{}'", rsp_content);
    let unwrap_rsp = ResponseWrap::from(rsp_content)?;
    Ok(
        unwrap_rsp.body()?
    )
}

///
pub(super) fn pure_fetch_newest_reserved_time_list(client: &reqwest::Client,
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
