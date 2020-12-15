use std::sync::atomic::{AtomicBool, Ordering};

static GLOBALLY_DISABLE_HOOK: AtomicBool = AtomicBool::new(false);

fn main() {
  assert_eq!(
    false,
    GLOBALLY_DISABLE_HOOK.compare_and_swap(false, true, Ordering::SeqCst)
  );
  assert_eq!(
    true,
    GLOBALLY_DISABLE_HOOK.compare_and_swap(true, false, Ordering::SeqCst)
  );
  assert_eq!(
    false,
    GLOBALLY_DISABLE_HOOK.compare_and_swap(true, false, Ordering::SeqCst)
  );
}
