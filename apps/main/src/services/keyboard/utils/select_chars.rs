use super::{trigger_event, trigger_event_ms};

use rdev::Key;

pub async fn select_chars(count: usize) -> Result<(), Box<dyn std::error::Error>> {
  // Trigger select and move to left char [Shift + LeftArrow]

  if count == 0 { return Ok(()); }

  trigger_event(Key::ShiftLeft, true).await?;

  for _ in 0..count {
    trigger_event_ms(Key::LeftArrow, true, 0).await?;
    trigger_event_ms(Key::LeftArrow, false, 0).await?;
  }

  trigger_event(Key::ShiftLeft, false).await?;

  Ok(())
}
