use rust_test_framework::{test_params, test_params_source};
use serde::Deserialize;

#[test_params(10)]
#[test_params(20)]
#[test_params_source(JsonFile("tests/test_data/test_built_in_types_u32.json"))]
fn test_mixed_u32(val: u32) {
    // test_built_in_types_u32.json contains [3]
    assert!(val == 10 || val == 20 || val == 3);
}

#[test_params("inline1")]
#[test_params_source(JsonString(r#"["source1", "source2"]"#))]
fn test_mixed_string(val: String) {
    assert!(val == "inline1" || val == "source1" || val == "source2");
}

#[derive(Deserialize, Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
}

#[test_params(User { name: "Alice", age: 30 })]
#[test_params_source(JsonString(r#"[{"name": "Bob", "age": 25}]"#))]
fn test_mixed_struct(user: User) {
    assert!(
        (user.name == "Alice" && user.age == 30) ||
        (user.name == "Bob" && user.age == 25)
    );
}

#[test_params(1, "one")]
#[test_params_source(JsonString(r#"[[2, "two"], [3, "three"]]"#))]
fn test_mixed_multiple_params(id: u32, label: String) {
    assert!(
        (id == 1 && label == "one") ||
        (id == 2 && label == "two") ||
        (id == 3 && label == "three")
    );
}

#[test_params(100)]
#[test_params_source(JsonFile("tests/test_data/test_built_in_types_u32.json"))]
#[test_params_source(JsonString("[200, 300]"))]
fn test_extremely_mixed(val: u32) {
    // test_built_in_types_u32.json contains [3]
    assert!(val == 100 || val == 3 || val == 200 || val == 300);
}

#[test_params_source(JsonFile("tests/test_data/test_single_object.json"))]
fn test_single_object_file(user: User) {
    assert_eq!(user.name, "Single");
    assert_eq!(user.age, 50);
}

#[test_params_source(JsonString(r#"{"name": "StringSingle", "age": 40}"#))]
fn test_single_object_string(user: User) {
    assert_eq!(user.name, "StringSingle");
    assert_eq!(user.age, 40);
}

#[test_params_source(JsonFile("tests/test_data/test_single_object.json"))]
#[test_params_source(JsonFile("tests/test_data/test_multiple_age_50_users.json"))]
#[test_params(("Peter", 50))]
#[test_params(User { age: 50, name: "Patrick" })]
#[test_params(User { name: "Richard", age: 50 },
              User { name: "Robert", age: 50 })]
#[test_params_source(JsonString(r#"{"name": "StringSingle", "age": 50}"#))]
#[test_params_source(JsonString(r#"[
                                    {"name": "John", "age": 50},
                                    {"name": "Jane", "age": 50}
                                   ]"#))]
fn test_multiple_same_type_mixed_sources(user: User) {
    assert_eq!(user.age, 50);
}
