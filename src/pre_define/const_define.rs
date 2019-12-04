/*
  create at 2019/12/4 by 'itachy'
*/
// most general inhere
pub mod general {
    pub const DEFAULT_LOG4RS_PATH: &str = "conf/log4rs.conf";
}

// network relation
pub mod network {
    pub mod headers {
        pub const USER_AGENT: &str = "PCDriving/2.25 (iPhone; iOS 11.3.1; Scale/2.00)";
        pub const HOST: &str = "appwebsrv01.22168168.com";
    }

    pub mod param_keys {
        pub const APP_PASSWORD:     &str = "AppPassword";
        pub const APP_PHONE_NUMBER: &str = "AppPhoneNumber";
        pub const NONCE:            &str = "nonce";
        pub const SIGNATURE:        &str = "signature";
        pub const TIMESTAMP:        &str = "timestamp";
        pub const APP_RECORD_ID:    &str = "AppRecordID";
        pub const PHASE:            &str = "Phase";
        pub const RESERVED_DATE:    &str = "ReservedDate";
        pub const TEACH_ID:         &str = "TeachID";
    }

    pub const REQ_DEFAULT_TIME_OUT_SECOND: u64 = 3;
}

// private define, TODO: need detach into config file
pub mod account_info {
    // 登录密码
    pub const APP_PASSWORD: &str = "";
    // 登录手机号
    pub const APP_PHONE_NUMBER: &str = "";
    // 上午1 下午2 晚上3
    pub const PHASE: &str = "";

    // TODO: 应该提取默认当天?
    // yyyy-mm-dd
    pub const RESERVED_DATE: &str = "";

    /* 没法不抓包获取的那些参数 */
    // 教练ID似乎是?
    pub const TEACH_ID: &str = "";
    pub const APP_RECORD_ID: &str = "";
}