use serde::Serialize;
use utoipa::ToSchema;

pub trait IntoCommonResponse {
    fn into_common_response_data(self) -> CommonResponse;
}

impl<T> IntoCommonResponse for T
where
    T: Serialize,
{
    fn into_common_response_data(self) -> CommonResponse {
        CommonResponse {
            code: 0,
            data: serde_json::to_value(self).expect("Failed to convert to serde_json::Value"),
            message: String::from("Success"),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CommonResponse {
    pub code: i16,
    pub data: serde_json::Value,
    pub message: String,
}

impl Default for CommonResponse {
    fn default() -> Self {
        CommonResponse {
            code: 0,
            data: serde_json::Value::Null,
            message: String::from("Success"),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CommonError {
    pub code: i16,
    pub message: String,
}

impl Into<CommonError> for (i16, &str) {
    fn into(self) -> CommonError {
        CommonError {
            code: self.0,
            message: String::from(self.1),
        }
    }
}
