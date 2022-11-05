use axum::BoxError;

pub type Result<T> = std::result::Result<T, BoxError>;
