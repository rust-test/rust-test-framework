use rust_test_framework::{test_params, test_params_source};

#[test_params_source(JsonFile("rust_test_framework/tests/test_data/test_built_in_types_u32.json"))]
#[test_params(3)]
fn test_duplicate_mixed(x: i32) {
    assert!(x > 0);
}

fn main() {}
