#[macro_export]
macro_rules! chk_send_err {
    ($expr:expr, $src:expr) => {
        if let Err(_e) = $expr {
            tracing::error!("Failed to send response \"{}\"", $src);
        }
    };
}

#[macro_export]
macro_rules! chk_err {
    ($expr:expr) => {
        if let Err(err) = $expr {
            tracing::error!("{err}");
        }
    };
}
