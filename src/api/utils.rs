use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseWrapper<T> {
    pub status: i32,
    pub data: Option<T>,
}