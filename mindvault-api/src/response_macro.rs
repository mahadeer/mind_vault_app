#[macro_export]
macro_rules! handle_service_response {
    (
        $service_call:expr,
        $success_msg_template:literal,
        $success_msg_args:expr,
        $error_msg_template:literal
    ) => {
        match $service_call {
            Ok(data) => {
                let success_arg = $success_msg_args(&data);
                tracing::debug!($success_msg_template, success_arg);
                Ok(axum::response::Json(data))
            },
            Err(err) => {
                let error_message = format!("{}: {:?}", $error_msg_template, err);
                tracing::error!("{}", error_message);
                Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, error_message))
            }
        }
    };
}