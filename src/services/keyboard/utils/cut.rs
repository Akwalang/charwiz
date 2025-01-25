use super::trigger_event;

use rdev::Key;

pub async fn cut() -> Result<(), Box<dyn std::error::Error>> {
  // Trigger paste [Ctrl + X]

  trigger_event(Key::ControlLeft, true).await?;
  trigger_event(Key::KeyX, true).await?;
  trigger_event(Key::KeyX, false).await?;
  trigger_event(Key::ControlLeft, false).await?;

  Ok(())
}
