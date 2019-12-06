/*
  create at 2019/12/3 by 'itachy'
*/
use crate::convenience::traits::*;


#[derive(Debug, serde::Deserialize)]
pub struct ApiCodeRsp {
    #[serde(rename = "IsSuccessed")]
    is_succeed: bool,

    #[serde(rename = "Message")]
    message: String,

    #[serde(rename = "Data")]
    data: ApiCodeRspData,
}

#[derive(Debug, serde::Deserialize)]
pub struct ApiCodeRspData {
    signature: String,
    timestamp: u64,
    nonce: u64,
}

impl ApiCodeRsp {
    pub fn data(self) -> ApiCodeRspData {
        self.data
    }
}


/// impl concerned
impl ApiCodeRspData {
    pub fn signature(&self) -> &str {
        &self.signature
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }
}

impl RequestStatus for ApiCodeRsp {
    fn is_req_succeed(&self) -> bool {
        self.is_succeed
    }
}