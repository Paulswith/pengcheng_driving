/*
  create at 2019/12/4 by 'itachy'
*/
mod request_combine;
use crate::convenience::{errors, tools};
use crate::network::basic;
use crate::data_model::{ResponseWrap,
                        api_code::*};
use crate::pre_define::api_define::*;


pub fn handle_entry() {
    match entry() {
        Err(err) => error!("Handle error: {}", err),
        Ok(_) => info!("Handle normal."),
    }
    //TEST CODE:
//    let uri_login = request_combine::combine_login_uri(GET_LOGIN,"IAMSIGNATUREIAMSIGNATUREIAMSIGNATUREIAMSIGNATURE");
//    info!("LoginURI: {}", uri_login);
}

fn entry() -> errors::Result<()> {
    let client = basic::construct_client()?;
    let api_code = request_api_code(&client)?;
    info!("signature: {}", api_code.signature());
    Ok(())
}


fn request_api_code(client: &reqwest::Client) -> errors::Result<ApiCodeRsp> {
    let ref url = request_combine::combine_get_api_code_uri(GET_API_CODE);
    let mut rsp = client.get(url).send()?;
    let ref rsp_content = rsp.text()?;
    debug!("Response body text: '{}'", rsp_content);
    let ref body = tools::unwrap_body(rsp_content)?;
    Ok(serde_json::from_str(body)?)
}
