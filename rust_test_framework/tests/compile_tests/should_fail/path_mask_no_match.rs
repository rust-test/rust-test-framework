use rust_test_framework::test_params_source;
use rust_test_framework::SourceType::PathMask;
use std::path::Path;

#[test_params_source(PathMask("non_existent_path/*.rs"))]
fn test_no_match(path: &Path) {
    let _ = path;
}

fn main() {}
