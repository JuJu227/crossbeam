use std::thread::sleep;
use std::time::Duration;
use std::u32;

use crossbeam_utils::sync::{Parker, UnparkReason};
use crossbeam_utils::thread;

#[test]
fn park_timeout_unpark_before() {
    let p = Parker::new();
    for _ in 0..10 {
        p.unparker().unpark();
        assert_eq!(
            p.park_timeout(Duration::from_millis(u32::MAX as u64)),
            UnparkReason::Unparked,
        );
    }
}

#[test]
fn park_timeout_unpark_not_called() {
    let p = Parker::new();
    for _ in 0..10 {
        assert_eq!(
            p.park_timeout(Duration::from_millis(10)),
            UnparkReason::Timeout,
        );
    }
}

#[test]
fn park_timeout_unpark_called_other_thread() {
    for _ in 0..10 {
        let p = Parker::new();
        let u = p.unparker().clone();

        thread::scope(|scope| {
            scope.spawn(move |_| {
                sleep(Duration::from_millis(50));
                u.unpark();
            });

            assert_eq!(
                p.park_timeout(Duration::from_millis(u32::MAX as u64)),
                UnparkReason::Unparked,
            );
        })
        .unwrap();
    }
}
