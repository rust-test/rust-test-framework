use rust_test_framework::test_params_source;
use std::path::Path;

#[test_params_source(rust_test_framework::SourceType::PathMask("tests/compile_tests/**/*.rs"))]
fn test_ui_files_exist(path: &Path) {
    assert!(path.exists());
    assert!(path.to_str().unwrap().ends_with(".rs"));
}
