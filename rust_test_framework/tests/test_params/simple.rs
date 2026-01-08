use rust_test_framework::{test_params, test_fixture, setup, teardown};

#[test_params(1)]
#[test_params(2)]
#[test_params(3)]
fn test_simple(val: u32) {
    assert!(val > 0);
}

#[test_params("a")]
#[test_params("b")]
#[test_params("c")]
fn test_string(val: String) {
    assert!(!val.is_empty());
}

#[test_params(1, "one")]
#[test_params(2, "two")]
fn test_multiple_values(id: u32, label: String) {
    assert!(id > 0);
    assert!(!label.is_empty());
}

#[test_params(Ok(1))]
#[test_params(Err("error"))]
fn test_result(val: Result<u32, String>) {
    if val.is_ok() {
        assert_eq!(val.unwrap(), 1);
    } else {
        assert_eq!(val.unwrap_err(), "error");
    }
}

#[test_params(Some(1))]
#[test_params(None)]
fn test_option(val: Option<u32>) {
    if val.is_some() {
        assert_eq!(val.unwrap(), 1);
    } else {
        assert!(val.is_none());
    }
}

#[rust_test_framework::test_params_source(JsonFile("tests/test_data/test_built_in_types_u32.json"))]
#[test_params(100)]
#[test_params(200)]
fn test_mixed_stacking(val: u32) {
    assert!(val > 0);
}

#[test_fixture]
mod fixture_with_test_params {
    use super::*;
    use std::sync::{LazyLock, Mutex};
    use std::time::Duration;
    use rust_test_framework::{test_params_source, wait_for};

    struct TestCounts {
        setup: u32,
        teardown: u32,
    }

    struct FixtureSync {
        lock: Mutex<TestCounts>,
    }

    static COUNTS: LazyLock<FixtureSync> = LazyLock::new(|| {
        FixtureSync {
            lock: Mutex::new(TestCounts { setup: 0, teardown: 0 }),
        }
    });

    #[setup]
    fn set_up() {
        println!("setup");
        let counts_sync = &*COUNTS;
        let mut counts = counts_sync.lock.lock().unwrap();
        counts.setup += 1;
    }

    #[teardown]
    fn tear_down() {
        println!("teardown");
        let counts_sync = &*COUNTS;
        let mut counts = counts_sync.lock.lock().unwrap();
        counts.teardown += 1;
    }

    #[test_params(1)]
    #[test_params(2)]
    #[test_params(3)]
    fn test_in_fixture(val: u32) {
        println!("{}", val);
        assert!(val > 0);
        let counts_sync = &*COUNTS;
        assert!(counts_sync.lock.lock().unwrap().setup > 0);
    }

    #[test_params_source(JsonString("[4]"))]
    fn test_in_fixture_sourced(val: u32) {
        println!("{}", val);
        assert!(val > 0);
        let counts_sync = &*COUNTS;
        assert!(counts_sync.lock.lock().unwrap().setup > 0);
    }

    #[test]
    fn verify_counts() {
        //TODO: add timeout attribute
        let counts_sync = &*COUNTS;
        let timeout = Duration::from_secs(5);
        let poll_interval = Duration::from_millis(500);

        let final_counts = wait_for!(
            || {
                let counts = counts_sync.lock.lock().unwrap();
                if counts.setup == 5 && counts.teardown == 4 {
                    Some(counts)
                } else {
                    None
                }
            },
            timeout,
            poll_interval,
            {
                let counts = counts_sync.lock.lock().unwrap();
                format!("waiting for test counts. Current: setup={}, teardown={}", counts.setup, counts.teardown)
            }
        );

        assert_eq!(final_counts.setup, 5);
        assert_eq!(final_counts.teardown, 4);
    }
}
