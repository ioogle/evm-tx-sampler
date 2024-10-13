use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseWrapper<T> {
    pub status: i32, // 0: failed, 1: success
    pub error_message: Option<String>,
    pub data: Option<T>,
}