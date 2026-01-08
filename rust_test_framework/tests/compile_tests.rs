use rust_test_framework::test_params;
use std::path::Path;
use std::sync::{LazyLock, Mutex};

static TRYBUILD: LazyLock<Mutex<trybuild::TestCases>> = LazyLock::new(|| {
    Mutex::new(trybuild::TestCases::new())
});

#[test_params("tests/ui/fixture_success.rs")]
#[test_params("tests/ui/qualified_paths.rs")]
fn ui_compile_should_pass(path: &Path) {
    TRYBUILD.lock().unwrap().pass(path);
}

#[test_params("tests/ui/setup_missing.rs")]
#[test_params("tests/ui/teardown_missing.rs")]
#[test_params("tests/ui/path_not_found.rs")]
#[test_params("tests/ui/duplicate_params_stacked.rs")]
#[test_params("tests/ui/duplicate_params_single.rs")]
#[test_params("tests/ui/duplicate_source_internal.rs")]
#[test_params("tests/ui/duplicate_source_mixed.rs")]
#[test_params("tests/ui/duplicate_enum.rs")]
#[test_params("tests/ui/duplicate_struct.rs")]
#[test_params("tests/ui/invalid_json_string.rs")]
fn ui_should_compile_fail(path: &Path) {
    TRYBUILD.lock().unwrap().compile_fail(path);
}
