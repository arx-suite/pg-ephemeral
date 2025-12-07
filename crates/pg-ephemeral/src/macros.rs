#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => { tracing::trace!($($arg)*) };
}

#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => { tracing::debug!($($arg)*) };
}

#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => { tracing::info!($($arg)*) };
}

#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => { tracing::warn!($($arg)*) };
}

#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => { tracing::error!($($arg)*) };
}
