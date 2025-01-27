use super::trigger_event;

use rdev::Key;

pub async fn select_word() -> Result<(), Box<dyn std::error::Error>> {
  // Trigger move to word start [Control + LeftArrow]

  trigger_event(Key::ControlLeft, true).await?;
  trigger_event(Key::LeftArrow, true).await?;
  trigger_event(Key::LeftArrow, false).await?;
  // Keep Control pressed

  // Trigger select and move to line end [Shift + RightArrow]

  trigger_event(Key::ShiftLeft, true).await?;
  trigger_event(Key::RightArrow, true).await?;
  trigger_event(Key::RightArrow, false).await?;
  trigger_event(Key::ShiftLeft, false).await?;

  // Release Control
  trigger_event(Key::ControlLeft, false).await?;

  Ok(())
}
