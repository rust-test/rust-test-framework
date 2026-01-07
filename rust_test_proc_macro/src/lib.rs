use proc_macro::TokenStream;
use rust_test_core::attributes;
#[allow(unused_imports)]
use rust_test_core::SourceType as SourceType;

/// Generates tests based on a provided source and model of that data
/// (must implement/derive `serde::Deserialize` or be a built-in type).
/// # Arguments
/// - `source_type`: A [`SourceType`] variant 
/// can be fully qualified or via just the variant name.
/// # Example
/// ```rust
/// # use rust_test_proc_macro as rust_test_framework;
/// use rust_test_framework::test_case_source;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct User { age: u32 }
///
/// #[test_case_source(JsonFile("test_data.json"))]
/// fn test_age_is_higher_then_zero(item: User) {
///     assert!(item.age > 0);
/// }
/// ```
#[proc_macro_attribute]
pub fn test_case_source(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::test_case_source(attr.into(), item.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn setup(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::setup(attr.into(), item.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn test_fixture(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::test_fixture(attr.into(), item.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
