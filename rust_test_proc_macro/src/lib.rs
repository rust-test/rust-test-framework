use proc_macro::TokenStream;
use rust_test_core::attributes;
#[allow(unused_imports)]
use rust_test_core::SourceType;

/// Generates tests based on a provided source and model of that data
/// (must implement/derive `serde::Deserialize` or be a build-in type).
/// # Arguments
/// - `source_type`: A [`SourceType`] variant 
/// can be fully qualified or via just the variant name.
/// # Example
/// ```rust
/// use rust_test::test_case_source;
/// use rust_test::SourceType;
///
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct User {
///     age: u32,
/// }
///
/// #[test_case_source(JsonFile("tests/test_ddt_data.json"))]
/// fn test_age_is_higher_then_zero(item: User) {
///     // test logic here
///     assert!(item.age > 0);
/// }
/// ```
#[proc_macro_attribute]
pub fn test_case_source(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::test_case_source(attr.into(), item.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
