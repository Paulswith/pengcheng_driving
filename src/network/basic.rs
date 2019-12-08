/*
  create at 2019/12/4 by 'itachy'
*/
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use reqwest::header::{USER_AGENT, HOST};
use crate::pre_define::const_define::network::{headers, REQ_DEFAULT_TIME_OUT_SECOND};


/// construct a default client
pub fn construct_client() -> reqwest::Result<reqwest::Client> {
    let headers = fit_request_header();
    let timeout = Duration::from_secs(REQ_DEFAULT_TIME_OUT_SECOND);
    reqwest::ClientBuilder::new()
        .timeout(timeout)
        .default_headers(headers)
        .build()
}

/// provide a fit header
pub fn fit_request_header() -> HeaderMap {
    let mut header = HeaderMap::new();
    add_header(&mut header, USER_AGENT, headers::USER_AGENT);
    add_header(&mut header, HOST, headers::HOST);
    header
}

/// add single header
fn add_header(header: &mut HeaderMap, key: HeaderName, value: &str) {
    match HeaderValue::from_str(value) {
        Err(err) => warn!("Fail parse: {} header-key: {} from value: {}", err, key, value),
        Ok(value) => { header.insert(key, value); },
    }
}

