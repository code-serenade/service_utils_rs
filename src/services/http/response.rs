use serde::Serialize;
use utoipa::ToSchema;

type Empty = ();

pub type CommonOk = CommonResponse<Empty>;

pub trait IntoCommonResponse<T>
where
    T: Serialize + ToSchema,
{
    fn into_common_response_data(self) -> CommonResponse<T>;
}

impl<T> IntoCommonResponse<T> for T
where
    T: Serialize + ToSchema,
{
    fn into_common_response_data(self) -> CommonResponse<T> {
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

impl Into<CommonError> for (i16, &str) {
    fn into(self) -> CommonError {
        CommonError {
            code: self.0,
            message: String::from(self.1),
        }
    }
}
