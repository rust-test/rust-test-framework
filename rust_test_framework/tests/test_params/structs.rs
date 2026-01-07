use rust_test_framework::test_params;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct CustomUser {
    name: String,
    age: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Dimensions {
    width: u32,
    height: u32,
}

#[test_params("Alice", 30)]
#[test_params("Bob", 25)]
fn test_custom_struct_inferred(name: String, age: u32) {
    assert!(!name.is_empty());
    assert!(age > 0);
}

#[test_params("Alice", 30)]
#[test_params("Bob", 25)]
fn test_custom_struct_single_arg(user: CustomUser) {
    assert!(!user.name.is_empty());
    assert!(user.age > 0);
}

#[test_params(Point { x: 1, y: 2 })]
fn test_struct_initialization(p: Point) {
    assert_eq!(p.x, 1);
    assert_eq!(p.y, 2);
}

#[test_params((1, 2), (10, 20))]
#[test_params((-5, 10), (100, 50))]
fn test_two_structs(p: Point, d: Dimensions) {
    assert!(p.x != 0 || p.y != 0);
    assert!(d.width > 0 && d.height > 0);
}

#[test_params(Point { x: 1, y: 2 }, Dimensions { width: 10, height: 20 })]
fn test_two_structs_initialization(p: Point, d: Dimensions) {
    assert_eq!(p.x, 1);
    assert_eq!(p.y, 2);
    assert_eq!(d.width, 10);
    assert_eq!(d.height, 20);
}
