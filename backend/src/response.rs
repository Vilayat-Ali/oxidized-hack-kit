#[macro_export]
macro_rules! response {
    ($status_code:expr, $message:expr, $data:tt) => {
        Json(json!({
            "status": $status_code,
            "message": $message,
            "data": json!($data)
        }))
    };

    ($status_code:expr, $message:expr) => {
        Json(json!({
            "status": $status_code,
            "message": $message
        }))
    };
}
