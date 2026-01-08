use rust_test_framework::test_params;
use std::path::Path;


#[test_params("tests/ui/fixture_success.rs")]
#[test_params("tests/ui/qualified_paths.rs")]
fn ui_compile_should_pass(path: &Path) {
    let t = trybuild::TestCases::new();
    t.pass(path);
}

#[test_params("tests/ui/setup_missing.rs")]
#[test_params("tests/ui/teardown_missing.rs")]
#[test_params("tests/ui/path_not_found.rs")]
fn ui_should_compile_fail(path: &Path) {
    let t = trybuild::TestCases::new();
    t.compile_fail(path);
}
