/*
  create at 2019/12/4 by 'itachy'
*/
mod request;
mod request_combine;
use crate::convenience::{errors, tools};
use crate::network::basic;
use crate::data_model::{ResponseWrap,
                        api_code::*,
                        init_config::*};
use crate::pre_define::api_define::*;


pub fn handle_entry(config: &InitConfig) {
    match basic::construct_client() {
        Err(err) => error!("Construct client failed: {}", err),
        Ok(ref client) => {
            if request::login(client, config) {
                println!("Login success.");
            }
            // 获取想要的预约的时间点的record-id
        }
    }

//    match entry() {
//        Err(err) => error!("Handle error: {}", err),
//        Ok(_) => info!("Handle normal."),
//    }
    //TEST CODE:
//    let uri_login = request_combine::combine_login_uri(GET_LOGIN,"IAMSIGNATUREIAMSIGNATUREIAMSIGNATUREIAMSIGNATURE");
//    info!("LoginURI: {}", uri_login);
}

