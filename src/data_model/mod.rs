/*
  create at 2019/12/3 by 'itachy'
*/
pub mod init_config;
pub mod api_code;
pub mod login;
use serde::Deserialize;
use crate::convenience::errors;


pub struct ResponseWrap(String);

impl ResponseWrap {
    fn new(raw: &str) -> Self {
        Self(raw.to_string())
    }

    pub fn from(wrap_content: &str) -> errors::Result<Self> {
        match dummy_xml::parser::parse_str(wrap_content) {
            Ok(ref document) if document.root().first_child().is_some() => {
                let child = document.root().first_child().unwrap();
                Ok(
                    ResponseWrap::new(child.value())
                )
            }
            _ => bail!(errors::ErrorKind::XmlParseFailed(format!("{:?}", wrap_content))),
        }
    }

    /// parse body by passing struct
    pub fn body<'a, T:'a>(&'a self) -> errors::Result<T> where T: Deserialize<'a> {
        debug!("Unwrap body: '{}'", &self.0);
        Ok(serde_json::from_str(&self.0)?)
    }
}
