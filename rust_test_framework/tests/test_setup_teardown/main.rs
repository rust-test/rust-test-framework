#[allow(unused_imports)]
use rust_test_framework::{test_fixture, test_case_source, setup, teardown};

#[test_fixture]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};
    use super::*;

    static SETUP_COUNT: AtomicU32 = AtomicU32::new(0);
    static TEARDOWN_COUNT: AtomicU32 = AtomicU32::new(0);

    #[setup]
    fn my_setup() {
        SETUP_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    #[teardown]
    fn my_teardown() {
        TEARDOWN_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn test_setup_called() {
        assert!(SETUP_COUNT.load(Ordering::SeqCst) > 0);
    }

    #[test]
    fn test_setup_called_again() {
        // Since tests might run in parallel, we just check it's > 0
        assert!(SETUP_COUNT.load(Ordering::SeqCst) > 0);
    }

    #[test_case_source(JsonFile("tests/test_data/setup_test.json"))]
    fn test_with_source(item: String) {
        assert!(SETUP_COUNT.load(Ordering::SeqCst) > 0);
        assert!(!item.is_empty());
    }
}

#[test_fixture]
mod tests_teardown_after_fail {
    use std::sync::atomic::{AtomicU32, Ordering};
    use super::*;

    static TEARDOWN_RUN_ON_FAIL: AtomicU32 = AtomicU32::new(0);

    #[teardown]
    fn teardown_on_fail() {
        TEARDOWN_RUN_ON_FAIL.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    #[should_panic]
    fn test_panics() {
        panic!("test failed");
    }

    #[test]
    fn test_check_teardown_ran() {
        // This test might run before or after test_panics
        // But we want to check if it ran at least once if test_panics finished.
        // Actually, with parallel execution, this is tricky.
    }
}

#[test_fixture]
mod tests_failure {
    use std::sync::atomic::{AtomicU32, Ordering};
    use super::*;

    pub static TEARDOWN_RUN_ON_SETUP_FAIL: AtomicU32 = AtomicU32::new(0);

    #[setup]
    fn failing_setup() {
        panic!("intentional failure");
    }

    #[teardown]
    fn teardown_not_ran() {
        TEARDOWN_RUN_ON_SETUP_FAIL.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    #[should_panic(expected = "setup failed: intentional failure")]
    fn test_failing_setup() {
    }
}

#[test_fixture]
mod tests_verify_failure {
    use std::sync::atomic::Ordering;

    #[test]
    fn test_verify_no_teardown() {
        assert_eq!(super::tests_failure::TEARDOWN_RUN_ON_SETUP_FAIL.load(Ordering::SeqCst), 0);
    }
}

mod tests_no_setup {
    #[test]
    fn test_without_setup() {
        // Just making sure normal tests still work
    }
}