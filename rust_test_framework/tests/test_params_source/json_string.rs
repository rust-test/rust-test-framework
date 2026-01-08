use rust_test_framework::test_params_source;

#[test_params_source(JsonString("[1, 2, 3]"))]
fn test_json_string_primitive(val: u32) {
    assert!(val > 0);
}

#[derive(serde::Deserialize, Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
}

#[test_params_source(JsonString(r#"[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]"#))]
fn test_json_string_struct(user: User) {
    assert!(!user.name.is_empty());
    assert!(user.age > 0);
}

#[test_params_source(JsonString(r#"{"name": "Charlie", "age": 35}"#))]
fn test_json_string_single_struct(user: User) {
    assert_eq!(user.name, "Charlie");
    assert_eq!(user.age, 35);
}

#[test_params_source(JsonString(r#"[[1, "one"], [2, "two"]]"#))]
fn test_json_string_tuple(id: u32, label: String) {
    assert!(id > 0);
    assert!(!label.is_empty());
}

#[test_params_source(JsonString("[[1, 2], [3, 4]]"))]
fn test_json_string_vec_of_vec(v: Vec<u32>) {
    assert!(!v.is_empty());
}

#[test_params_source(JsonString(r#""A""#))]
fn test_json_string_single_enum(e: TestEnum) {
    assert_eq!(e, TestEnum::A);
}

#[derive(serde::Deserialize, Debug, PartialEq)]
enum TestEnum {
    A,
    B,
    C(u32),
}

#[test_params_source(JsonString(r#"[{"name": "Alice", "age": 30}]"#))]
fn test_json_string_vec_of_struct(user: User) {
    assert_eq!(user.name, "Alice");
}

#[test_params_source(JsonString(r#"["A", "B", {"C": 10}]"#))]
fn test_json_string_vec_enum(e: TestEnum) {
    match e {
        TestEnum::A => (),
        TestEnum::B => (),
        TestEnum::C(10) => (),
        _ => (),
    }
}

#[test_params_source(JsonString("[null, 1]"))]
fn test_json_string_option(v: Option<u32>) {
    assert!(v.is_none() || v.is_some());
}
