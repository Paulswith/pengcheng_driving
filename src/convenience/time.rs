/*
  create at 2019/12/4 by 'itachy'
*/
use std::time::SystemTime;


/// 根据每天搞事的时间, 返回一个精准的时间戳
pub fn cal_timestamp_at_today() -> u64 {
    //TODO: 这里需要返回一个今天18点的时间戳
    1575453600
}

/// 从配置加载, 返回每天搞事的时间点
pub fn time_at_day() -> String {
    //TODO: 抽离为配置加载
    String::from("18:00:00")
}

/// 返回当前时间的时间戳
pub fn cur_timestamp() -> Option<u64> {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Err(err) => {
            error!("Unable convert local timestamp");
            None
        }
        Ok(timestamp) => Some(timestamp.as_secs()),
    }
}