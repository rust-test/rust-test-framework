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

### Requirements

- **Rust Version**: 1.70.0 or higher
- **Edition**: 2021

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
rust_test_framework = "0.1.1-alpha.9"
```

Example usage:

### Inlined Parameters

Use `#[test_params]` to provide test cases directly in your code. You can stack multiple attributes for multiple test
cases.

```rust
use rust_test_framework::test_params;

#[test_params(1, "one")]
#[test_params(2, "two")]
fn test_multiple_params(id: u32, label: &str) {
  assert!(id > 0);
  assert!(!label.is_empty());
}
```

#### Advanced Types and Rust-style Initialization

`test_params` supports idiomatic Rust syntax for structs, enums, `Option`, and `Result`.

```rust
use rust_test_framework::test_params;
use serde::Deserialize;

#[derive(Deserialize)]
struct Point {
  x: i32,
  y: i32
}

#[derive(Deserialize, Debug, PartialEq)]
enum Kind {
  Small,
  Large(u32),
}

#[test_params(Point{ x: 1, y: 2 })]
fn test_struct(p: Point) {
  assert_eq!(p.x, 1);
}

#[test_params(Kind::Small)]
#[test_params(Kind::Large(100))]
fn test_enum(kind: Kind) {
  // ...
}

#[test_params(Some(42))]
#[test_params(None)]
fn test_option(val: Option<u32>) {
  // ...
}
```

### External Data Sources

Use `#[test_params_source]` to load test cases from external files.

```rust
use rust_test_framework::{test_params_source, SourceType};
use serde::Deserialize;

#[derive(Deserialize)]
struct TestCase {
  a: i32,
  b: i32,
  expected: i32,
}

// This will generate a test case for each entry in `tests/data.json` 
// if it's a list or inject it as a single entry if it's an object.
#[test_params_source(JsonFile("tests/data.json"))]
fn test_addition(case: TestCase) {
  assert_eq!(case.a + case.b, case.expected);
}

// You can also use multiple parameters with external sources.
// Each entry in the JSON array should then be an array of values.
#[test_params_source(JsonFile("tests/multi_params.json"))]
fn test_multi(id: u32, name: String) {
  assert!(id > 0);
}
```

### Mixing Inline Parameters and External Sources

You can combine `#[test_params]` and `#[test_params_source]` to run a test with data from multiple sources.

```rust
use rust_test_framework::{test_params, test_params_source};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct User {
  name: String,
  age: u32,
}

#[test_params_source(JsonFile("tests/user_single.json"))]
#[test_params_source(JsonFile("tests/users_list.json"))]
#[test_params(("Peter", 50))]
#[test_params(User { age: 50, name: "Patrick" })]
#[test_params(User { name: "Richard", age: 50 },
              User { name: "Robert", age: 50 })]
#[test_params_source(JsonString(r#"{"name": "StringSingle", "age": 50}"#))]
#[test_params_source(JsonString(r#"[
                                    {"name": "John", "age": 50},
                                    {"name": "Jane", "age": 50}
                                   ]"#))]
fn test_multiple_sources(user: User) {
  assert_eq!(user.age, 50);
}
```

### Test Fixtures

Use `#[test_fixture]` on a module to enable `#[setup]` and `#[teardown]` functions.

```rust
use rust_test_framework::{test_fixture, setup, teardown};

#[test_fixture]
mod my_tests {
  use super::*;

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
