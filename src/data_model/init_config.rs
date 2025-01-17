/*
  create at 2019/12/6 by 'itachy'
*/
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
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
    ticket_rush_time: u32,
    heart_beat_interval: u32,
    order_start_time: u32,
    beforehand_order_second: u32,
}

impl Config {
    pub fn phone_number(&self) -> &str {
        &self.authentication.phone_number
    }

    pub fn password(&self) -> &str {
        &self.authentication.password
    }

    pub fn app_record_id(&self) -> &str {
        &self.application.app_record_id
    }

    pub fn order_start_time(&self) -> String {
        format!("{}:00:00", self.application.order_start_time)
    }

    pub fn hour_of_ticket_rush_time(&self) -> u32 {
        self.application.ticket_rush_time
    }

//    pub fn hour_of_order_start_time(&self) -> u32 {
//        self.application.order_start_time
//    }

    pub fn heart_beat_interval(&self) -> u32 {
        self.application.heart_beat_interval
    }

    pub fn beforehand_order_second(&self) -> u32 { self.application.beforehand_order_second }

    pub fn phase(&self) -> String {
        match self.application.order_start_time {
            6..=12  => 1.to_string(),
            13..=18 => 2.to_string(),
            19..=22 => 3.to_string(),
            _ => {
                error!("Error special order_start_time: {}", self.application.order_start_time);
                0.to_string()
            }
        }
    }
}
