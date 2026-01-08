use rust_test_framework::test_params;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
enum MyEnum {
    A,
    B,
}

#[test_params(MyEnum::A)]
#[test_params(MyEnum::A)]
fn test_duplicate_enum(e: MyEnum) {
    assert!(matches!(e, MyEnum::A));
}

fn main() {}
