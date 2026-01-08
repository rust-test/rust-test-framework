use rust_test_framework::test_params;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
struct MyStruct {
    x: i32,
}

#[test_params(MyStruct { x: 1 })]
#[test_params(MyStruct { x: 1 })]
fn test_duplicate_struct(s: MyStruct) {
    assert_eq!(s.x, 1);
}

fn main() {}
