use rust_test_framework::test_params_source;
use std::path::Path;
use std::sync::{LazyLock, Mutex};

static TRYBUILD: LazyLock<Mutex<trybuild::TestCases>> =
    LazyLock::new(|| Mutex::new(trybuild::TestCases::new()));

mod should_pass {
    use super::*;

    #[test_params_source(PathMask("tests/compile_tests/should_pass/*.rs"))]
    fn ui_compile_should_pass(path: &Path) {
        TRYBUILD.lock().unwrap().pass(path);
    }
}

mod should_fail {
    use super::*;

    #[test_params_source(PathMask("tests/compile_tests/should_fail/*.rs"))]
    fn ui_should_compile_fail(path: &Path) {
        TRYBUILD.lock().unwrap().compile_fail(path);
    }
}
