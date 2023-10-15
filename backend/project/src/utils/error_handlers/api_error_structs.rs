use chrono::{DateTime, Local};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug)]
pub enum ErrorType {
    DbError,
    NotFound,
    Forbidden,
    Unauthorized,
    Validation,
    Grand,
}

#[derive(Debug)]
pub struct ApiError {
    pub message: String,
    pub type_error: ErrorType,
}

#[derive(Serialize, ToSchema)]
pub struct ApiErrorResponse {
    pub message: String,
    pub info: String,
    pub date_time: DateTime<Local>,
}