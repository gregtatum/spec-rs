use std::sync::atomic::{AtomicBool, Ordering};

static GLOBALLY_DISABLE_HOOK: AtomicBool = AtomicBool::new(false);

#[test]
fn test_atomics() {
    assert_eq!(
        false,
        GLOBALLY_DISABLE_HOOK
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .expect("GLOBALLY_DISABLE_HOOK compare_exchange 1"),
        "First compare exchnage is false"
    );
    assert_eq!(
        true,
        GLOBALLY_DISABLE_HOOK
            .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
            .expect("GLOBALLY_DISABLE_HOOK compare_exchange 2"),
        "Second compare exchnage is true"
    );
    assert_eq!(
        false,
        GLOBALLY_DISABLE_HOOK
            .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
            .expect_err("GLOBALLY_DISABLE_HOOK compare_exchange err 3"),
        "Third compare exchange was an error, as the value was not true"
    );
}
