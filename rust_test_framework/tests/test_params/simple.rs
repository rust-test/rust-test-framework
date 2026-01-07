use rust_test_framework::{test_params, test_fixture, setup, teardown};

#[test_params(1)]
#[test_params(2)]
#[test_params(3)]
fn test_simple(val: u32) {
    assert!(val > 0);
}

#[test_params("a")]
#[test_params("b")]
#[test_params("c")]
fn test_string(val: String) {
    assert!(!val.is_empty());
}

#[test_params(10)]
#[test_params(20)]
fn test_no_turbofish(val: u32) {
    assert!(val >= 10);
}

#[test_params(1, "one")]
#[test_params(2, "two")]
fn test_multiple_values(id: u32, label: String) {
    assert!(id > 0);
    assert!(!label.is_empty());
}

#[test_params(1)]
#[test_params(2)]
fn test_stacked(val: u32) {
    assert!(val > 0);
}

#[test_params(Ok(1))]
#[test_params(Err("error"))]
fn test_result(val: Result<u32, String>) {
    if val.is_ok() {
        assert_eq!(val.unwrap(), 1);
    } else {
        assert_eq!(val.unwrap_err(), "error");
    }
}

#[test_params(Some(1))]
#[test_params(None)]
fn test_option(val: Option<u32>) {
    if val.is_some() {
        assert_eq!(val.unwrap(), 1);
    } else {
        assert!(val.is_none());
    }
}

#[rust_test_framework::test_params_source(JsonFile("tests/test_data/test_built_in_types_u32.json"))]
#[test_params(100)]
#[test_params(200)]
fn test_mixed_stacking(val: u32) {
    assert!(val > 0);
}

#[test_fixture]
mod fixture_with_test_params {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static SETUP_COUNT: AtomicU32 = AtomicU32::new(0);

    #[setup]
    fn set_up() {
        SETUP_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    #[teardown]
    fn tear_down() {
        // Just for completeness
    }

    #[test_params(1)]
    #[test_params(2)]
    #[test_params(3)]
    fn test_in_fixture(val: u32) {
        assert!(val > 0);
        assert!(SETUP_COUNT.load(Ordering::SeqCst) > 0);
    }

    #[test]
    fn verify_counts() {
        assert!(SETUP_COUNT.load(Ordering::SeqCst) > 0);
    }
}
