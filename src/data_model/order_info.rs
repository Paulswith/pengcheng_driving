/*
create at 2019/12/8 by itachy
*/
use crate::convenience::traits::*;


#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename = "Root")]
pub struct OrderInfo {
    #[serde(rename = "IsSuccessed")]
    is_succeed: bool,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Data")]
//    data: Vec<Daum>,
    data: ::serde_json::Value, // ignore result display
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Daum {
    #[serde(rename = "RecordID")]
    record_id: String,
    #[serde(rename = "ReservedID")]
    reserved_id: String,
    #[serde(rename = "StudentNO")]
    student_no: String,
    #[serde(rename = "AppRecordID")]
    app_record_id: String,
    #[serde(rename = "TeacherNo")]
    teacher_no: String,
    #[serde(rename = "Course")]
    course: String,
    #[serde(rename = "OrderDateTime")]
    order_date_time: String,
    #[serde(rename = "GradeType")]
    grade_type: Option<String>,
    #[serde(rename = "GradeContent")]
    grade_content: Option<String>,
    #[serde(rename = "TeacherStar")]
    teacher_star: Option<String>,
    #[serde(rename = "SchoolStar")]
    school_star: Option<String>,
    #[serde(rename = "State")]
    state: String,
    #[serde(rename = "TrainingItem")]
    training_item: Option<String>,
    #[serde(rename = "Mileage")]
    mileage: Option<String>,
    #[serde(rename = "Remark")]
    remark: String,
    #[serde(rename = "DeviceType")]
    device_type: String,
    #[serde(rename = "CancelDate")]
    cancel_date: Option<String>,
    #[serde(rename = "CancelRemark")]
    cancel_remark: Option<String>,
    #[serde(rename = "CreatedOn")]
    created_on: String,
    #[serde(rename = "IsValid")]
    is_valid: String,
    #[serde(rename = "ReviewDate")]
    review_date: Option<String>,
    #[serde(rename = "ReservedHours")]
    reserved_hours: i64,
    #[serde(rename = "ReservedHoursInto")]
    reserved_hours_into: Option<String>,
    #[serde(rename = "SignInDateTime")]
    sign_in_date_time: Option<String>,
    #[serde(rename = "SignOutDateTime")]
    sign_out_date_time: Option<String>,
    #[serde(rename = "PayNum")]
    pay_num: Option<String>,
    #[serde(rename = "PayDate")]
    pay_date: Option<String>,
    #[serde(rename = "PayState")]
    pay_state: String,
    #[serde(rename = "PayModeNO")]
    pay_mode_no: Option<String>,
    #[serde(rename = "IPAddr")]
    ipaddr: Option<String>,
    #[serde(rename = "IsSync")]
    is_sync: String,
    #[serde(rename = "SyncDate")]
    sync_date: Option<String>,
    #[serde(rename = "SerialNo")]
    serial_no: String,
    #[serde(rename = "StartTime")]
    start_time: String,
    #[serde(rename = "EndTime")]
    end_time: String,
}

impl RequestStatus for OrderInfo {
    fn is_req_succeed(&self) -> bool {
        self.is_succeed
    }
}