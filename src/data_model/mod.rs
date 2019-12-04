/*
  create at 2019/12/3 by 'itachy'
*/
pub mod api_code;
use serde::Deserialize;


/*  parse from xml */
#[derive(Debug, serde::Deserialize)]
pub struct ResponseWrap {
    string: String,         // contain one ApiCodeRspCore

    #[serde(rename = "version", default)]
    version: String,
}


impl ResponseWrap {
    pub fn body<'a, T:'a>(&'a self) -> Option<T> where T: Deserialize<'a> {
        match serde_json::from_str(&self.string) {
            Err(err) => {
                error!("Convert error: {}, from \n{}", err, self.string);
                None
            },
            Ok(body) => Some(body),
        }
    }
}
