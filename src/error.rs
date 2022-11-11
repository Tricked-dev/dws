use axum::{
    body::BoxBody,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<BoxBody> {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!( {
                "error": self.0.to_string()
            })),
        )
            .into_response()
    }
}

#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return Err($crate::error::AppError(anyhow::anyhow!($msg)))
    };
}

pub type Result<T> = std::result::Result<T, AppError>;
