use super::trigger_event;

use rdev::Key;

pub async fn select_line() -> Result<(), Box<dyn std::error::Error>> {
  // Trigger move to line start [Home]

  trigger_event(Key::Home, true).await?;
  trigger_event(Key::Home, false).await?;

  // Trigger select and move to line end [Shift + End]

  trigger_event(Key::ShiftLeft, true).await?;
  trigger_event(Key::End, true).await?;
  trigger_event(Key::End, false).await?;
  trigger_event(Key::ShiftLeft, false).await?;

  Ok(())
}
