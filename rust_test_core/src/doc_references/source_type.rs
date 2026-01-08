use std::marker::PhantomData;
use serde::de::DeserializeOwned;

/// A source type to generate tests from.
///
/// - `T`: The type to deserialize into. If omitted, the macro attempts to
/// infer it from the function signature.
///
/// # Variants:
/// - [`JsonFile::<T>(path)`](SourceType::JsonFile): A path to a JSON file.
/// - [`JsonString::<T>(json)`](SourceType::JsonString): A JSON string literal.
/// - [`PathMask(pattern)`](SourceType::PathMask): A glob pattern to match files.
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

    /// # Example
    /// ```rust
    /// # use rust_test_core::SourceType;
    /// # use serde::Deserialize;
    /// # #[derive(Deserialize)]
    /// # struct User { name: String, age: u32 }
    /// # let user_source: SourceType<User> =
    /// // Type inferred from the function signature:
    /// SourceType::JsonString(r#"{"name": "Alice", "age": 30}"#)
    /// # ;
    /// # let users_source =
    /// // Type explicitly provided,
    /// // can also be used as JsonString::<Vec<User>>(r#"[...]"#)
    /// SourceType::<Vec<User>>::JsonString(r#"[{"name": "Alice", "age": 30}]"#)
    /// # ;
    /// ```
    JsonString(&'static str),

    /// A glob pattern to match files.
    ///
    /// It generates a test for each file matching the pattern.
    /// The test function must have exactly one parameter of type `&Path` or `PathBuf`.
    ///
    /// # Example
    /// ```rust
    /// # use rust_test_core::SourceType;
    /// # use std::path::Path;
    /// # let source: SourceType<()> =
    /// SourceType::PathMask("tests/ui/*.rs")
    /// # ;
    /// ```
    PathMask(&'static str),

    #[doc(hidden)]
    __PrivateMarker(PhantomData<T>)
}
