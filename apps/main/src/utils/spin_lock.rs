use std::time::{Duration, Instant};

// TODO: Remove it. Even few ms delay affect typing
pub fn spin_sleep(duration: Duration) {
  let start = Instant::now();

  while start.elapsed() < duration {
    std::hint::spin_loop();
  }
}
