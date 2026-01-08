use rust_test_framework::wait_for;
use std::time::{Duration, Instant};

#[test]
fn test_wait_for_success_immediate() {
    let result = wait_for!(
        || Some(42),
        Duration::from_secs(1),
        Duration::from_millis(10),
        "should not time out"
    );
    assert_eq!(result, 42);
}

#[test]
fn test_wait_for_success_after_delay() {
    let start = Instant::now();
    let result = wait_for!(
        || {
            if start.elapsed() >= Duration::from_millis(100) {
                Some("done")
            } else {
                None
            }
        },
        Duration::from_secs(1),
        Duration::from_millis(10),
        "should not time out"
    );
    assert_eq!(result, "done");
    assert!(start.elapsed() >= Duration::from_millis(100));
}

#[test]
#[should_panic(expected = "Timed out after")]
fn test_wait_for_timeout() {
    wait_for!(
        || None::<()>,
        Duration::from_millis(50),
        Duration::from_millis(10),
        "custom panic message"
    );
}

#[test]
fn test_wait_for_timeout_message_content() {
    let result = std::panic::catch_unwind(|| {
        wait_for!(
            || None::<()>,
            Duration::from_millis(50),
            Duration::from_millis(10),
            "check message"
        );
    });

    let panic_msg = result.err().and_then(|b| {
        b.downcast_ref::<String>()
            .cloned()
            .or_else(|| b.downcast_ref::<&str>().map(|s| s.to_string()))
    }).expect("Should have panicked with a message");

    assert!(panic_msg.contains("Timed out after"));
    assert!(panic_msg.contains("checks"));
    assert!(panic_msg.contains("poll interval: 10ms"));
    assert!(panic_msg.contains("check message"));
}

#[test]
fn test_wait_for_multiple_polls() {
    let mut counter = 0;
    let result = wait_for!(
        || {
            counter += 1;
            if counter >= 5 {
                Some(counter)
            } else {
                None
            }
        },
        Duration::from_secs(1),
        Duration::from_millis(1),
        "should not time out"
    );
    assert_eq!(result, 5);
    assert!(counter >= 5);
}

