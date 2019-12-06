/*
  create at 2019/12/4 by 'itachy'
*/
mod request;
mod uri_combine;
use crate::convenience::{errors, tools};
use crate::network::basic;
use crate::data_model::{ResponseWrap,
                        api_code::*,
                        init_config::*};
use crate::pre_define::api_define::*;


pub fn handle_entry(config: &Config) {
    match basic::construct_client() {
        Err(err) => error!("Construct client failed: {}", err),
        Ok(ref client) => {
            handle_by_step(client, config);
        }
    }
}

///
fn handle_by_step(client: &reqwest::Client, config: &Config) {
    // step 1. login
    if !request::login(client, config) {
        error!("Login failed");
        return;
    }
    info!("Login succeed.");

    // step 2. fetch teach_id
    let teach_info = request::fetch_teach_info(client, config);
    if teach_info.is_none() {
        error!("Fetch teach_info failed.");
        return;
    }
    info!("Fetch teach_info succeed.");

    // step 3. fetch reserved.record_id by config
    let ref teach_info = teach_info.unwrap();
    let reserved = request::fetch_newest_reserved_time_list(client, config, teach_info);
    if reserved.is_none() {
        error!("Fetch reserved failed.");
        return;
    }
    info!("Fetch newest_reserved_time_list succeed.");
    println!("{:?}", reserved.unwrap());
    // step 4. loop, until  the time point is approaching, request order quickly.
}
