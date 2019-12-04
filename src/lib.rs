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
extern crate serde_xml_rs;
use pre_define::const_define::general;


pub fn entry() {
    app_log4rs_init();
}

/// setup log4rs from conf file:
fn app_log4rs_init() {
    log4rs::init_file(general::DEFAULT_LOG4RS_PATH,Default::default())
        .expect("Initialize log config error");
}
