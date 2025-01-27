use super::trigger_event;

use rdev::Key;

pub async fn select_all() -> Result<(), Box<dyn std::error::Error>> {
  // Trigger select all [Ctrl + A]

  trigger_event(Key::ControlLeft, true).await?;
  trigger_event(Key::KeyA, true).await?;
  trigger_event(Key::KeyA, false).await?;
  trigger_event(Key::ControlLeft, false).await?;

  Ok(())
}
