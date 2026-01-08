use rust_test_framework::test_params;

#[test_params(1, 1)]
fn test_duplicate_in_one_attr(x: i32) {
    assert_eq!(x, 1);
}

fn main() {}
