use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
    message: String,
    date_time: DateTime<Local>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String,
    info: String,
    date_time: DateTime<Local>,
}

impl MessageResponse {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            date_time: Local::now(),
        }
    }
}

impl ErrorResponse {
    pub fn new(info: &str) -> Self {
        Self {
            message: "Error".to_string(),
            info: info.to_string(),
            date_time: Local::now(),
        }
    }
}
