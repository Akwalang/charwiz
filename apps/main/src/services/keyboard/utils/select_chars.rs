use super::{trigger_event, trigger_event_ms};

use rdev::Key;

pub async fn select_chars(count: usize) -> Result<(), Box<dyn std::error::Error>> {
  select_chars_limited(count, 100usize).await
}

pub async fn select_chars_limited(count: usize, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
  // Trigger select and move to left char [Shift + LeftArrow]

  if count == 0 { return Ok(()); }

  let count = count.min(limit);

  trigger_event(Key::ShiftLeft, true).await?;

  for _ in 0..count {
    trigger_event_ms(Key::LeftArrow, true, 0).await?;
    trigger_event_ms(Key::LeftArrow, false, 0).await?;
  }

  trigger_event(Key::ShiftLeft, false).await?;

  Ok(())
}
