use rust_test_framework::test_params;

#[test_params(1)]
#[test_params(1)]
fn test_duplicate_simple(x: i32) {
    assert_eq!(x, 1);
}

fn main() {}
