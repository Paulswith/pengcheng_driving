/*
create at 2019/12/6 by itachy
*/
use crate::convenience::traits::*;


#[derive(Debug, Clone, serde::Deserialize)]
pub struct Reserved {
    #[serde(rename = "IsSuccessed")]
    is_succeed: bool,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Data")]
    data: Data,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Data {
    #[serde(rename = "ReservedTime")]
    reserved_time: Vec<ReservedTime>,
    #[serde(rename = "ExaminList")]
    examin_list: Vec<ExaminList>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ReservedTime {
    #[serde(rename = "RecordID")]
    record_id: String,
    #[serde(rename = "TeacherID")]
    teacher_id: String,
    #[serde(rename = "ReservedDate")]
    reserved_date: String,
    #[serde(rename = "ReservedTime")]
    reserved_time: String,
    #[serde(rename = "StartTime")]
    start_time: String,
    #[serde(rename = "EndTime")]
    end_time: String,
    #[serde(rename = "Number")]
    number: i64,
    #[serde(rename = "IsValid")]
    is_valid: String,
    #[serde(rename = "UsedNumber")]
    used_number: i64,
    #[serde(rename = "IsMyUsed")]
    is_my_used: i64,
    #[serde(rename = "HoursPrice")]
    hours_price: f64,
    #[serde(rename = "ValidMemo")]
    valid_memo: Option<String>,
    #[serde(rename = "CarOrderID")]
    car_order_id: Option<String>,
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Tel")]
    tel: Option<String>,
    #[serde(rename = "IdentityCard")]
    identity_card: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ExaminList {
    #[serde(rename = "ExaminType")]
    examin_type: String,
    #[serde(rename = "Name")]
    name: String,
}

impl ReservedTime {
    pub fn record_id(&self) -> &str {
        &self.record_id
    }

//    pub fn valid_memo(&self) -> &str {
//        &self.valid_memo
//    }
}


impl Reserved {
    pub fn reserved_id(&self, by_start_time: &str) -> Option<String> {
        for record in self.data.reserved_time.iter() {
            match &record.car_order_id {
                Some(car_order_id) => return Some(car_order_id.to_owned()),
                _ => (),
            }
        }
        None
    }
}

impl RequestStatus for Reserved {
    fn is_req_succeed(&self) -> bool {
        self.is_succeed
    }
}