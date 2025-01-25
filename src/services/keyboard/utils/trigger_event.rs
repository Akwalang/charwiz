use std::time::Duration;

use async_std::task::sleep;
use rdev::{simulate, EventType, Key};

pub async fn trigger_event(key: Key, is_press: bool) -> Result<(), rdev::SimulateError> {
  let event = match is_press {
    true => EventType::KeyPress(key),
    false => EventType::KeyRelease(key),
  };

  simulate(&event)?;

  sleep(Duration::from_millis(5)).await;

  Ok(())
}
