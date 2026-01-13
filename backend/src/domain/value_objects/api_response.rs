use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    success: bool,
    payload: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(success: bool, payload: T) -> Self {
        Self { success, payload }
    }

    pub fn success(payload: T) -> Self {
        Self {
            success: true,
            payload,
        }
    }
}

impl ApiResponse<String> {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            payload: message.into(),
        }
    }
}
