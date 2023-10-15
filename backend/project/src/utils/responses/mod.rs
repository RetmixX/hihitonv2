use chrono::{DateTime, Local};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct MessageResponse {
    message: String,
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
