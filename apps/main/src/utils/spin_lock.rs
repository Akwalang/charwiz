use std::time::{Duration, Instant};

pub fn spin_sleep(duration: Duration) {
  let start = Instant::now();

  while start.elapsed() < duration {
    std::hint::spin_loop();
  }
}
