/*
create at 2019/12/6 by itachy
*/
use crate::convenience::traits::*;


#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
pub struct TeachInfo {
    #[serde(rename = "IsSuccessed")]
    is_succeed: bool,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Data")]
    data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
struct Data {
    #[serde(rename = "TeachNo")]
    teach_no: String,
}

impl TeachInfo {
    pub fn teach_no(&self) -> &str {
        &self.data.teach_no
    }
}

impl RequestStatus for TeachInfo {
    fn is_req_succeed(&self) -> bool {
        self.is_succeed
    }
}

