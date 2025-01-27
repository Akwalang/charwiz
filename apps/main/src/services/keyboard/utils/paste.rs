use super::trigger_event;

use rdev::Key;

pub async fn paste() -> Result<(), Box<dyn std::error::Error>> {
  // Trigger paste [Ctrl + V]

  trigger_event(Key::ControlLeft, true).await?;
  trigger_event(Key::KeyV, true).await?;
  trigger_event(Key::KeyV, false).await?;
  trigger_event(Key::ControlLeft, false).await?;

  Ok(())
}
