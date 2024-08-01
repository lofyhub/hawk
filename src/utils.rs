use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum SuccessData<T> {
    Single(T),
    Multiple(Vec<T>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessResponse<T> {
    success: bool,
    data: SuccessData<T>,
}

impl<T> SuccessResponse<T> {
    pub fn new_single(data: T) -> Self {
        SuccessResponse {
            success: true,
            data: SuccessData::Single(data),
        }
    }
    pub fn new_multiple(data: Vec<T>) -> Self {
        SuccessResponse {
            success: true,
            data: SuccessData::Multiple(data),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse<T> {
    success: bool,
    error: T,
}
impl<T> ErrorResponse<T> {
    pub fn new(data: T) -> Self {
        ErrorResponse {
            success: false,
            error: data,
        }
    }
}
