use rust_test_framework::test_params_source;

#[test_params_source(JsonFile("rust_test_framework/tests/test_data/duplicate.json"))]
fn test_duplicate_source(x: i32) {
    assert!(x > 0);
}

fn main() {}
