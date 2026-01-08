pub use rust_test_proc_macro::{
    setup, teardown, test_fixture, test_params, test_params_source, rust_test_seen_value,
};
pub use rust_test_core::SourceType;

/// Returns the version of the framework.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// A macro that waits for a condition to be met, polling at a regular interval.
///
/// This macro is useful for testing asynchronous operations where you need to wait for a state change.
/// It will repeatedly execute the provided `$condition` closure until it returns `Some(value)` or the `$timeout` is reached.
///
/// # Arguments
///
/// * `$condition`: A closure that returns `Option<T>`. When it returns `Some(T)`, the macro returns that value.
/// * `$timeout`: A `std::time::Duration` specifying the maximum time to wait.
/// * `$poll_interval`: A `std::time::Duration` specifying how often to poll the condition.
/// * `$msg`: A message to include in the panic if the macro times out.
///
/// # Panics
///
/// Panics if the `$timeout` is reached before the `$condition` returns `Some(T)`.
/// The panic message includes the total elapsed time, the number of checks performed, the poll interval, and the provided `$msg`.
///
/// # Example
///
/// ```rust
/// use rust_test_framework::wait_for;
/// use std::time::Duration;
///
/// let result = wait_for!(
///     || Some(42),
///     Duration::from_secs(1),
///     Duration::from_millis(10),
///     "should not time out"
/// );
/// assert_eq!(result, 42);
/// ```
#[macro_export]
macro_rules! wait_for {
    ($condition:expr, $timeout:expr, $poll_interval:expr, $msg:expr) => {{
        let start = std::time::Instant::now();
        let mut checks = 0;
        loop {
            checks += 1;
            if let Some(res) = $condition() {
                break res;
            }
            let elapsed = start.elapsed();
            if elapsed > $timeout {
                panic!(
                    "Timed out after {:?} ({} checks, poll interval: {:?}): {}",
                    elapsed, checks, $poll_interval, $msg
                );
            }
            std::thread::sleep($poll_interval);
        }
    }};
}

#[doc(hidden)]
pub mod __private {
    pub use serde_json;
}