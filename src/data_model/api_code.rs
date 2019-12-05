/*
  create at 2019/12/3 by 'itachy'
*/
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

/// only impl concerned
impl ApiCodeRsp {
    pub fn signature(&self) -> &str {
        &self.data.signature
    }
}
