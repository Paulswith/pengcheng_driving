/*
  create at 2019/12/4 by 'itachy'
*/
// most general inhere
pub mod general {
    pub const DEFAULT_LOG4RS_PATH: &str = "conf/log4rs.yaml";
    pub const DEFAULT_CONFIG_PATH: &str = "conf/conf.toml";
}

// network relation
pub mod network {
    pub mod headers {
        pub const USER_AGENT:         &str = "PCDriving/2.25 (iPhone; iOS 11.3.1; Scale/2.00)";
        pub const HOST:               &str = "appwebsrv01.22168168.com";
    }

    pub mod param_keys {
        pub const K_APP_PASSWORD:     &str = "AppPassword";
        pub const K_APP_PHONE_NUMBER: &str = "AppPhoneNumber";
        pub const K_NONCE:            &str = "nonce";
        pub const K_SIGNATURE:        &str = "signature";
        pub const K_TIMESTAMP:        &str = "timestamp";
        pub const K_APP_RECORD_ID:    &str = "AppRecordID";
        pub const K_RESERVED_ID:      &str = "ReservedID";
        pub const K_PHASE:            &str = "Phase";
        pub const K_RESERVED_DATE:    &str = "ReservedDate";
        pub const K_TEACH_ID:         &str = "TeachID";
    }

    pub const REQ_DEFAULT_TIME_OUT_SECOND: u64 = 1;
}

pub mod timer {
    // 每秒发几次
    pub const SEND_REQUEST_PER_SECOND: u32 = 1;

    // 超过多少秒之前还得发
    pub const SEND_REQUEST_BEFORE_SECONDS: u32 = 2;

    // 预防心跳导致抢票晚了
    pub const AVOID_TIMEOUT_FOR_HEART: i64 = 5;
}