# rust_test_framework

[![Crates.io](https://img.shields.io/crates/v/rust_test_framework.svg)](https://crates.io/crates/rust_test_framework)
[![Docs.rs](https://docs.rs/rust_test_framework/badge.svg)](https://docs.rs/rust_test_framework)
[![GitHub](https://img.shields.io/github/license/rust-test/rust-test-framework)](https://github.com/rust-test/rust-test-framework/blob/main/LICENSE)
[![GitHub](https://img.shields.io/badge/GitHub-rust--test%2Frust--test--framework-blue?logo=github)](https://github.com/rust-test/rust-test-framework)
[![Rust](https://github.com/rust-test/rust-test-framework/actions/workflows/rust.yml/badge.svg)](https://github.com/rust-test/rust-test-framework/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/rust-test/rust-test-framework/branch/main/graph/badge.svg)](https://codecov.io/gh/rust-test/rust-test-framework)
[![contribute](https://contribute.so/api/badge/github/v-dermichev)](https://contribute.so/github/v-dermichev)

A data-driven testing framework for Rust.

## Features

- **Data-Driven Testing (DDT)**: Run the same test logic with multiple inputs.
- **Test Fixtures**: Support for `setup` and `teardown` functions within a test module.
- **Procedural Macros**: Easy-to-use attributes for defining test cases and fixtures.
- **Clean Output**: Clear results for individual test cases.

## Usage

This project is currently in **alpha**.

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
rust_test_framework = "0.1.1-alpha.1"
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

// This will generate a test case for each entry in `tests/data.json` 
// if it's a list or inject it as a single entry if it's an object.
#[test_case_source(JsonFile("tests/data.json"))]
fn test_addition(case: TestCase) {
    assert_eq!(case.a + case.b, case.expected);
}
```

### Test Fixtures

Use `#[test_fixture]` on a module to enable `#[setup]` and `#[teardown]` functions.

```rust
use rust_test_framework::{test_fixture, setup, teardown};

#[test_fixture]
mod my_tests {
    #[setup]
    fn set_up() {
        // This runs before each test in the module
        println!("Setting up...");
    }

    #[teardown]
    fn tear_down() {
        // This runs after each test in the module
        println!("Tearing down...");
    }

    #[test]
    fn test_example() {
        assert!(true);
    }
}
```

## License

Licensed under the Apache License, Version 2.0.
