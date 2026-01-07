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
    /// # use rust_test_core::SourceType;
    /// # use serde::Deserialize;
    /// # #[derive(Deserialize)]
    /// # struct User { name: String, age: u32 }
    /// # let user_source: SourceType<User> =
    /// // Type inferred from the function signature:
    /// SourceType::JsonFile("data.json")
    /// # ;
    /// # let users_source =
    /// // Type explicitly provided,
    /// // can also be used as JsonFile::<Vec<User>>("data.json")
    /// SourceType::<Vec<User>>::JsonFile("data.json")
    /// # ;
    /// ```
    JsonFile(&'static str),
    #[doc(hidden)]
    __PrivateMarker(PhantomData<T>)
}
