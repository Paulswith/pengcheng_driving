/*
  create at 2019/12/6 by 'itachy'
*/
use crate::convenience::traits::*;


#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename="Root")]
pub struct Login {
    #[serde(rename = "IsSuccessed")]
    is_succeed: bool,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Data")]
    data: Vec<Daum>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Daum {
    #[serde(rename = "RecordID")]
    record_id: String,
    #[serde(rename = "UserID")]
    user_id: String,
    #[serde(rename = "UserName")]
    user_name: String,
    #[serde(rename = "SexCodeNO")]
    sex_code_no: String,
    #[serde(rename = "IdentityCard")]
    identity_card: String,
    #[serde(rename = "PhoneNumber")]
    phone_number: String,
    #[serde(rename = "Country")]
    country: String,
    #[serde(rename = "Birthday")]
    birthday: ::serde_json::Value,
    #[serde(rename = "UserType")]
    user_type: String,
    #[serde(rename = "Remark")]
    remark: ::serde_json::Value,
    #[serde(rename = "PhotoAddress")]
    photo_address: String,
    #[serde(rename = "IsStudent")]
    is_student: i64,
    #[serde(rename = "IsHoursModel")]
    is_hours_model: String,
    #[serde(rename = "DeptCodeNO")]
    dept_code_no: String,
    #[serde(rename = "DeptCodeName")]
    dept_code_name: String,
    #[serde(rename = "StudentNO")]
    student_no: String,
}

impl RequestStatus for Login {
    fn is_req_succeed(&self) -> bool {
        self.is_succeed
    }
}
