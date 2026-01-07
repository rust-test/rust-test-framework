pub use rust_test_proc_macro::{test_params, test_params_source, setup, teardown, test_fixture};
pub use rust_test_core::SourceType;

/// Returns the version of the framework.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[doc(hidden)]
pub mod __private {
    pub use serde_json;
}