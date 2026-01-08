use rust_test_framework::test_params_source;

#[test_params_source(JsonString("{ invalid json }", i32))]
fn test_invalid_json(x: i32) {
    assert!(x > 0);
}

fn main() {}
