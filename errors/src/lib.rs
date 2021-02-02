use thiserror::Error;

use abi::*;
use macros::{respone_into_result, result_into_vec};

/// General error definition for the project
///
/// TODO: please rename `MyError`
#[derive(Error, Debug)]
pub enum MyError {
    // detailed errors
    #[error("Unsupported API: {0}")]
    UnsupportedApi(String),
    #[error("Malformed API response for {0}")]
    MalformedApiResponse(String),

    // converted errors
    #[error("Protobuf decode error: {0}")]
    ProstDecodeError(#[from] prost::DecodeError),
    #[error("Protobuf decode error: {0}")]
    ProstEncodeError(#[from] prost::EncodeError),
}

impl From<MyError> for AppError {
    fn from(err: MyError) -> Self {
        AppError::new(get_code(&err), err.to_string())
    }
}

impl From<&AppError> for MyError {
    fn from(err: &AppError) -> Self {
        match AppErrorCode::from_i32(err.code).unwrap() {
            AppErrorCode::UnsupportedApi => MyError::UnsupportedApi(err.message.to_owned()),
            AppErrorCode::MalformedApiResponse => {
                MyError::MalformedApiResponse(err.message.to_owned())
            }

            // converted errors
            _ => unimplemented!(),
        }
    }
}

fn get_code(e: &MyError) -> AppErrorCode {
    match e {
        MyError::UnsupportedApi(_) => AppErrorCode::UnsupportedApi,
        MyError::MalformedApiResponse(_) => AppErrorCode::MalformedApiResponse,

        // converted errors
        MyError::ProstDecodeError(_) => AppErrorCode::ProstDecodeError,
        MyError::ProstEncodeError(_) => AppErrorCode::ProstEncodeError,
    }
}

/// convert protobuf type ResponseMsg into a Result<&Msg, &MyError>
pub trait IntoResult {
    /// internal type for the ResponseMsg
    type Msg;

    /// extract Msg or AppError to Result
    fn into_result(&self) -> Result<&Self::Msg, MyError>;
}

/// Convert Result<Msg, MyError> into protobuf bytes
pub trait IntoVec {
    /// generate protobuf bytes based on Result
    fn into_vec(&self) -> Vec<u8>;
}

respone_into_result!(ResponsePong, Pong);
result_into_vec!(ResponsePong, Pong);
