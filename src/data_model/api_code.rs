/*
  create at 2019/12/3 by 'itachy'
*/
#[derive(Debug, serde::Deserialize)]
struct ApiCodeRspCore {
    #[serde(rename = "IsSuccessed")]
    is_succeed: bool,

    #[serde(rename = "Message")]
    message: String,

    #[serde(rename = "@version")]
    data: ApiCodeRspCoreData,
}

#[derive(Debug, serde::Deserialize)]
struct ApiCodeRspCoreData {
    signature: String,
    timestamp: u64,
    nonce: u64,
}
