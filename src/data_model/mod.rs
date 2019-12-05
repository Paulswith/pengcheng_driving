/*
  create at 2019/12/3 by 'itachy'
*/
pub mod api_code;
use serde::Deserialize;
use crate::convenience::errors;

/*  parse from xml */
/*
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "string")]
pub struct ResponseWrap {
    #[serde(rename = "_")]
    body: String,                    // contain one ApiCodeRspCore

//    #[serde(rename = "@version")]
//    string: api_code::ApiCodeRsp,
}


impl ResponseWrap {
    pub fn body<'a, T:'a>(&'a self) -> errors::Result<T> where T: Deserialize<'a> {
        debug!("my body: '{}'", &self.body);
        Ok(serde_json::from_str(&self.body)?)
    }
}
*/
