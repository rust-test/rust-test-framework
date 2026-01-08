use rust_test_framework::test_params;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
enum TestEnum {
    A,
    B,
    C(u32),
    D { x: u32, y: u32 },
}

#[test_params(TestEnum::A)]
#[test_params(TestEnum::B)]
fn test_enum_unit(val: TestEnum) {
    assert!(val == TestEnum::A || val == TestEnum::B);
}

#[test_params(1, TestEnum::A)]
#[test_params(2, TestEnum::B)]
fn test_mixed_enum(id: u32, val: TestEnum) {
    assert!(id > 0);
    assert!(val == TestEnum::A || val == TestEnum::B);
}

#[test_params(TestEnum::C(3))]
fn test_enum_tuple(val: TestEnum) {
    assert_eq!(val, TestEnum::C(3));
}

#[test_params(TestEnum::D { x: 1, y: 2 })]
#[test_params(TestEnum::D { x: 3, y: 4 })]
fn test_enum_struct(val: TestEnum) {
    match val {
        TestEnum::D { x, y } => assert!((x == 1 && y == 2) || (x == 3 && y == 4)),
        _ => panic!("Expected TestEnum::D"),
    }
}
