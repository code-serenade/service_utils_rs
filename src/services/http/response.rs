use core::str;

use axum::{Json, http::StatusCode};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Default, Clone)]
pub struct Empty;

pub type CommonOk = CommonResponse<Empty>;

pub trait IntoCommonResponse<T>
where
    T: Serialize + ToSchema,
{
    fn into_common_response(self) -> CommonResponse<T>;
}

impl<T> IntoCommonResponse<T> for T
where
    T: Serialize + ToSchema,
{
    fn into_common_response(self) -> CommonResponse<T> {
        CommonResponse {
            code: 0,
            data: self,
            message: String::from("Success"),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CommonResponse<T>
where
    T: Serialize + ToSchema,
{
    pub code: i16,
    pub data: T,
    pub message: String,
}

impl<T> CommonResponse<T>
where
    T: Serialize + ToSchema,
{
    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

impl<T> Default for CommonResponse<T>
where
    T: Serialize + ToSchema + Default,
{
    fn default() -> Self {
        CommonResponse {
            code: 0,
            data: T::default(),
            message: String::from("Success"),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CommonError {
    pub code: i16,
    pub message: String,
}

impl CommonError {
    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

impl Into<CommonError> for (i16, &str) {
    fn into(self) -> CommonError {
        CommonError {
            code: self.0,
            message: String::from(self.1),
        }
    }
}

pub type ResponseResult<T> =
    core::result::Result<Json<CommonResponse<T>>, (StatusCode, Json<CommonError>)>;
pub type Result<T> = core::result::Result<T, (StatusCode, Json<CommonError>)>;
