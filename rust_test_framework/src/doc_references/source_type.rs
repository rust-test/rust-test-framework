use std::marker::PhantomData;
use serde::de::DeserializeOwned;

/// A source type to generate tests from.
///
/// - `T`: The type to deserialize into. If omitted, the macro attempts to
/// infer it from the function signature.
pub enum SourceType<T = ()>
where
    T: DeserializeOwned
{
    /// Pass a path to a JSON file.
    ///
    /// # Example
    /// ```
    /// // Inferred:
    /// #[test_case_source(JsonFile("data.json"))]
    /// fn my_test(data: User) { ... }
    ///
    /// // Explicit:
    /// #[test_case_source(SourceType::<User>::JsonFile("data.json"))]
    /// fn my_test<T: Debug>(data: T) { ... }
    /// ```
    JsonFile(&'static str, PhantomData<T>),
}
