use rust_test_framework::test_params_source;
use serde::Deserialize;
use std::any::{type_name, type_name_of_val};

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub age: u32,
}

#[test_params_source(JsonFile::<User>("tests/test_data/test_ddt_data.json"))]
fn test_users_explicit_type<T: Into<User>>(user: T) {
    let user: User = user.into();
    assert!(user.age > 0);
}

#[test_params_source(JsonFile("tests/test_data/test_built_in_types_u32.json"))]
fn test_built_in_types_u32(number: u32) {
    assert_eq!(type_name_of_val(&number), type_name::<u32>());
}

#[test_params_source(JsonFile("tests/test_data/test_built_in_types_string.json"))]
fn test_built_in_types_string(string: String) {
    assert!(!string.is_empty());
}

#[test_params_source(JsonFile::<u32>("tests/test_data/test_generic.json"))]
fn test_generic<T>(val: T)
where
    T: std::fmt::Debug,
{
    let debug_string = format!("{:?}", val);
    assert!(!debug_string.is_empty());
}

#[test_params_source(JsonFile("tests/test_data/test_ddt_data.json"))]
fn test_users_inferred(user: User) {
    assert!(user.age > 0);
}

#[test_params_source(JsonFile("tests/test_data/test_vec_of_vec.json"))]
fn test_vec_of_vec(v: Vec<u32>) {
    assert!(!v.is_empty());
}

#[test_params_source(JsonFile("tests/test_data/test_single_vec.json"))]
fn test_single_vec(v: Vec<u32>) {
    assert!(!v.is_empty());
}

#[test_params_source(JsonFile("tests/test_data/test_null.json"))]
fn test_null(v: Option<u32>) {
    assert!(v.is_none());
}

#[test_params_source(JsonFile("tests/test_data/test_bool.json"))]
fn test_bool(v: bool) {
    assert!(v.eq(&true) || v.eq(&false))
}

#[test_params_source(JsonFile("tests/test_data/test_empty_suffix.json"))]
fn test_empty_suffix(v: String) {
    assert!(v.trim().is_empty() || v == "!!");
}

#[test_params_source(JsonFile("tests/test_data/test_complex.json"))]
fn test_complex(v: serde_json::Value) {
    assert!(v.is_array() || v.is_object());
}

#[derive(Deserialize, Debug, PartialEq)]
enum TestEnum {
    A,
    B,
    C(u32),
    D { x: u32, y: u32 },
}

#[test_params_source(JsonFile("tests/test_data/test_enums.json"))]
fn test_enums(e: TestEnum) {
    match e {
        TestEnum::A => (),
        TestEnum::B => (),
        TestEnum::C(10) => (),
        TestEnum::D { x: 1, y: 2 } => (),
        _ => panic!("Unexpected enum variant: {:?}", e),
    }
}

#[test]
fn test_framework_version() {
    assert!(!rust_test_framework::version().is_empty());
}
