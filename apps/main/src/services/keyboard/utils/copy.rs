use super::trigger_event;

use rdev::Key;

pub async fn copy() -> Result<(), Box<dyn std::error::Error>> {
  // Trigger copy [Ctrl + C]

  trigger_event(Key::ControlLeft, true).await?;
  trigger_event(Key::KeyC, true).await?;
  trigger_event(Key::KeyC, false).await?;
  trigger_event(Key::ControlLeft, false).await?;

  Ok(())
}
