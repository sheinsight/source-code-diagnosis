#[macro_export]
macro_rules! debug_expensive {
    // 带 target 的版本
    (target: $target:expr, $fmt:expr, $($arg:expr),+ $(,)?) => {{
        if log::log_enabled!(target: $target, log::Level::Debug) {
            log::debug!(target: $target, $fmt, $($arg),+);
        }
    }};

    // 不带 target 的版本
    ($fmt:expr, $($arg:expr),+ $(,)?) => {{
        if log::log_enabled!(log::Level::Debug) {
            log::debug!($fmt, $($arg),+);
        }
    }};

    // 单个表达式的版本
    ($fmt:expr) => {{
        if log::log_enabled!(log::Level::Debug) {
            log::debug!($fmt);
        }
    }};
}
