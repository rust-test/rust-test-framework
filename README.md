# rust_test_framework

A data-driven testing framework for Rust.

## Features

- **Data-Driven Testing (DDT)**: Run the same test logic with multiple inputs.
- **Procedural Macros**: Easy to use attributes for defining test cases.
- **Clean Output**: Clear results for individual test cases.

## Usage

This project is currently in **alpha**.

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
rust_test_framework = "0.1.0-alpha.7"
```

Example usage:

```rust
use rust_test_framework::test_case_source;
use rust_test_framework::SourceType;
use serde::Deserialize;

#[derive(Deserialize)]
struct TestCase {
    a: i32,
    b: i32,
    expected: i32,
}

#[test_case_source(SourceType::JsonFile("tests/data.json", TestCase))]
fn test_addition(case: TestCase) {
    assert_eq!(case.a + case.b, case.expected);
}
```

## License

Licensed under the Apache License, Version 2.0.
