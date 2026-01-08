use rust_test_framework::test_params;
use std::path::Path;

#[test_params("non_existent_file.rs")]
fn test_path_exists(path: &Path) {
    let _ = path;
}

fn main() {}
