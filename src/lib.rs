/*
  create at 2019/12/3 by 'itachy'
*/
mod data_model;
mod convenience;
mod pre_define;
mod network;
mod handler;

#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;
extern crate serde;
extern crate toml;
use pre_define::const_define::general;
use convenience::errors;
use data_model::init_config::Config;


pub fn entry() {
    app_log4rs_init();
    let ref config = parse_config(general::DEFAULT_CONFIG_PATH)
        .expect("Unable parse config file");
    info!("Init config: {:?}", config);
    handler::handle_entry(config);
}

/// setup log4rs from conf file:
fn app_log4rs_init() {
    log4rs::init_file(general::DEFAULT_LOG4RS_PATH,Default::default())
        .expect("Initialize log config error");
}

fn parse_config(from_file_path: &str) -> errors::Result<Config> {
    let ref content = std::fs::read_to_string(from_file_path)?;
    Ok(
        toml::from_str(content)?
    )
}