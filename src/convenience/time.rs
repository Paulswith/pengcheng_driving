/*
  create at 2019/12/4 by 'itachy'
*/
use std::time::SystemTime;


/// 获取最新的预约日期
pub fn newest_orderable_date() -> String {
    //TODO: 自动生成 2d后
    String::from("2019-12-06")
}


/// 获取最新的预约timestamp
pub fn fetch_newest_apply_timestamp() -> u64 {
    //TODO: 根据配置的目标时间, 来生成
    1575626400
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
