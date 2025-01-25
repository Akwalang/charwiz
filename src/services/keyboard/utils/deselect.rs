use super::trigger_event;

use rdev::Key;

pub async fn deselect() -> Result<(), Box<dyn std::error::Error>> {
  // Trigger move right [RightArrow]

  trigger_event(Key::RightArrow, true).await?;
  trigger_event(Key::RightArrow, false).await?;

  Ok(())
}
