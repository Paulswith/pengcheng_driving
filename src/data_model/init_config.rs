/*
  create at 2019/12/6 by 'itachy'
*/
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitConfig {
    authentication: Authentication,
    application: Application,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Authentication {
    phone_number: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Application {
    app_record_id: String,
    order_start_time: String,
}

impl InitConfig {
    pub fn phone_number(&self) -> &str {
        &self.authentication.phone_number
    }

    pub fn password(&self) -> &str {
        &self.authentication.password
    }

    pub fn app_record_id(&self) -> &str {
        &self.application.app_record_id
    }

    pub fn order_start_time(&self) -> &str {
        &self.application.order_start_time
    }
}
