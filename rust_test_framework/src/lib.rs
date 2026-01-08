pub use rust_test_proc_macro::{
    setup, teardown, test_fixture, test_params, test_params_source, rust_test_seen_value,
};
pub use rust_test_core::SourceType;

/// Returns the version of the framework.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[doc(hidden)]
pub mod __private {
    pub use serde_json;
}