use rust_test_framework::test_params_source;
use std::path::Path;

#[test_params_source(rust_test_framework::SourceType::PathMask("tests/compile_tests/**/*.rs"))]
fn test_ui_files_recursive(path: &Path) {
    assert!(path.exists());
    assert!(path.to_str().unwrap().ends_with(".rs"));
    // Ensure it's inside tests/compile_tests
    assert!(path.to_str().unwrap().contains("tests/compile_tests/"));
}

#[test_params_source(rust_test_framework::SourceType::PathMask("../rust_test_core/src/**/*.rs"))]
fn test_core_rs_files(path: &Path) {
    assert!(path.exists());
    assert!(path.to_str().unwrap().ends_with(".rs"));
}

#[test_params_source(rust_test_framework::SourceType::PathMask("tests/compile_tests/**/*/[f-p]*.rs"))]
fn test_ui_files_range(path: &Path) {
    assert!(path.exists());
    let filename = path.file_name().unwrap().to_str().unwrap();
    let first_char = filename.chars().next().unwrap();
    assert!(first_char >= 'f' && first_char <= 'p');
}

#[test_params_source(rust_test_framework::SourceType::PathMask("tests/**/p*.rs"))]
fn test_multiple_wildcards(path: &Path) {
    assert!(path.exists());
    let filename = path.file_name().unwrap().to_str().unwrap();
    assert!(filename.starts_with('p'));
}
