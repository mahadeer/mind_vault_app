use axum::http::StatusCode;
use axum::Json;

pub(crate) type ApiResponse<T> = Result<Json<T>, (StatusCode, String)>;