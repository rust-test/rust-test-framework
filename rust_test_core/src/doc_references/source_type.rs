use std::marker::PhantomData;
use serde::de::DeserializeOwned;

/// A source type to generate tests from.
///
/// - `T`: The type to deserialize into. If omitted, the macro attempts to
/// infer it from the function signature.
/// # Variants:
/// - [`JsonFile::<T>(path)`](SourceType::JsonFile): A path to a JSON file.
pub enum SourceType<T: DeserializeOwned>
{
    /// # Example
    /// ```rust
    /// // Type inferred from the function signature:
    /// #[test_case_source(JsonFile("data.json"))]
    /// fn my_test(data: User) { ... }
    ///
    /// // Type explicitly provided:
    /// #[test_case_source(JsonFile::<Vec<Users>>("data.json"))]
    /// fn my_test<T: Debug>(data: T) { ... }
    /// ```
    JsonFile(&'static str),
    #[doc(hidden)]
    __PrivateMarker(PhantomData<T>)
}
