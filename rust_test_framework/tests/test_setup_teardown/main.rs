#[allow(unused_imports)]
use rust_test_framework::{test_fixture, test_case_source, setup};

#[test_fixture]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};
    use super::*;

    static SETUP_COUNT: AtomicU32 = AtomicU32::new(0);

    #[setup]
    fn my_setup() {
        SETUP_COUNT.fetch_add(1, Ordering::SeqCst);
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
mod tests_failure {
    use super::*;
    #[setup]
    fn failing_setup() {
        panic!("intentional failure");
    }

    #[test]
    #[should_panic(expected = "setup failed: intentional failure")]
    fn test_failing_setup() {
        // This should fail during setup
    }
}

mod tests_no_setup {
    #[test]
    fn test_without_setup() {
        // Just making sure normal tests still work
    }
}
