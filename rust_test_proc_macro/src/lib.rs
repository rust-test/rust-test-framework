use proc_macro::TokenStream;
use rust_test_core::attributes;
#[allow(unused_imports)]
use rust_test_core::SourceType as SourceType;

/// Generates tests based on provided inlined parameters.
/// (must implement/derive `serde::Deserialize` or be a built-in type).
/// # Example
/// ```rust
/// # use rust_test_proc_macro as rust_test_framework;
/// use rust_test_framework::test_params;
///
/// #[test_params(1)]
/// #[test_params(2)]
/// #[test_params(3)]
/// fn test_numbers(item: u32) {
///     assert!(item > 0);
/// }
/// ```
#[proc_macro_attribute]
pub fn test_params(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::test_params(attr.into(), item.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

/// Generates tests based on a provided source and model of that data
/// (must implement/derive `serde::Deserialize` or be a built-in type).
/// # Arguments
/// - `source_type`: A [`SourceType`] variant 
/// can be fully qualified or via just the variant name.
/// # Example
/// ```rust
/// # use rust_test_proc_macro as rust_test_framework;
/// use rust_test_framework::test_params_source;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct User { age: u32 }
///
/// #[test_params_source(JsonFile("test_data.json"))]
/// fn test_age_is_higher_then_zero(item: User) {
///     assert!(item.age > 0);
/// }
/// ```
#[proc_macro_attribute]
pub fn test_params_source(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::test_params_source(attr.into(), item.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

/// Marks a function as a setup function to be run before each test in a `#[test_fixture]`.
///
/// # Example
/// ```rust
/// # use rust_test_proc_macro as rust_test_framework;
/// use rust_test_framework::{test_fixture, setup};
///
/// #[test_fixture]
/// mod my_tests {
/// #   use rust_test_proc_macro::setup;
///     #[setup]
///     fn before_each() {
///         // setup logic here
///     }
///
///     #[test]
///     fn some_test() {
///         // ...
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn setup(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::setup(attr.into(), item.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

/// Marks a function as a teardown function to be run after each test in a `#[test_fixture]`.
///
/// # Example
/// ```rust
/// # use rust_test_proc_macro as rust_test_framework;
/// use rust_test_framework::{test_fixture, teardown};
///
/// #[test_fixture]
/// mod my_tests {
/// #   use rust_test_proc_macro::teardown;
///     #[teardown]
///     fn after_each() {
///         // teardown logic here
///     }
///
///     #[test]
///     fn some_test() {
///         // ...
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn teardown(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::teardown(attr.into(), item.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

/// Marks a module as a test fixture, enabling `#[setup]` and `#[teardown]` functionality.
///
/// # Example
/// ```rust
/// # use rust_test_proc_macro as rust_test_framework;
/// use rust_test_framework::{test_fixture, setup, teardown};
///
/// #[test_fixture]
/// mod my_tests {
/// #   use rust_test_proc_macro::{setup, teardown};
///     #[setup]
///     fn set_up() {
///         println!("Setting up...");
///     }
///
///     #[teardown]
///     fn tear_down() {
///         println!("Tearing down...");
///     }
///
///     #[test]
///     fn test_example() {
///         assert!(true);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn test_fixture(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::test_fixture(attr.into(), item.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
