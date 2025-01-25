use super::trigger_event_ms;

use rdev::Key;

pub async fn deselect() -> Result<(), Box<dyn std::error::Error>> {
  // Trigger move right [RightArrow]

  trigger_event_ms(Key::RightArrow, true, 0).await?;
  trigger_event_ms(Key::RightArrow, false, 0).await?;

  Ok(())
}
