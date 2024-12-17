#[macro_export]
macro_rules! chk_send {
    ($expr:expr, $src:expr) => {
        if let Err(_e) = $expr {
            tracing::error!("Failed to send response \"{}\"", $src);
        }
    };
}
