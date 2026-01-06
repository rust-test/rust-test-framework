use rust_test_core::attributes;
use proc_macro::TokenStream;

/// Generates tests based on a provided source and model of that data 
/// (must implement `serde::Deserialize` or be a build-in type).
/// # Arguments
/// - `source_type`: A [`rust_test::SourceType`] enum value.
/// # Example
/// ```rust
/// use rust_test::test_case_source;
///
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct User {
///     age: u32,
/// }
///
/// #[test_case_source(SourceType::JsonFile("tests/test_ddt_data.json", User))]
/// fn test_age_is_higher_then_zero(item: User) {
///     // test logic here
///     assert!(item.age > 0);
/// }
/// ```
#[proc_macro_attribute]
pub fn test_case_source(attr: TokenStream, item: TokenStream) -> TokenStream {
    attributes::test_case_source(attr.into(), item.into()).into()
}
