/*
  create at 2019/12/4 by 'itachy'
*/
mod request_combine;
use crate::network::basic;
use crate::pre_define::api_define::*;


pub fn handle_entry() {
    match basic::construct_client() {
        Err(err) => {
            error!("Init request client failed");
        },
        Ok(client) => {

        }
    }
    //TEST CODE:
    let uri_login = request_combine::combine_login_uri(GET_LOGIN,"IAMSIGNATUREIAMSIGNATUREIAMSIGNATUREIAMSIGNATURE");
    info!("LoginURI: {}", uri_login);
}

fn convenience_signature(client: &reqwest::Client) -> Option<String> {
    match client.get(request_combine::combine_get_api_code_uri(GET_API_CODE)).send() {
        Err(err) => {
            error!("Error when request api_code: {}", err);
            None
        },
        Ok(body) => {
            //TODO: BODY TO STRUCT
        }
    }
}