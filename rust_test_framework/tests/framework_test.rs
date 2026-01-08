use rust_test_framework;

#[test]
fn test_framework_version() {
    assert!(!rust_test_framework::version().is_empty());
}
