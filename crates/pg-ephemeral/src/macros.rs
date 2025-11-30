#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => { tracing::trace!($($arg)*) };
}

#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => { tracing::debug!($($arg)*) };
}

#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => { tracing::info!($($arg)*) };
}

#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => { tracing::warn!($($arg)*) };
}

#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => { tracing::error!($($arg)*) };
}

#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {};
}
