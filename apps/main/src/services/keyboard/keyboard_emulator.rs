use std::time::Duration;

use async_std::task::sleep;
use rdev::{simulate, SimulateError, EventType, Key};

use crate::services::config::Config;

pub struct KeyboardEmulator {
  long_delay: u64,
  medium_delay: u64,
  short_delay: u64,
}

impl KeyboardEmulator {
  pub fn new() -> Self {
    let config = Config::get_instance();

    let delays = config.get_delay();

    KeyboardEmulator {
      long_delay: delays.long,
      medium_delay: delays.medium,
      short_delay: delays.short,
    }
  }

  pub async fn copy(&self) -> anyhow::Result<()> {
    // Trigger copy [Ctrl + C]

    self.trigger_medium_event(Key::ControlLeft, true).await?;
    self.trigger_medium_event(Key::KeyC, true).await?;
    self.trigger_medium_event(Key::KeyC, false).await?;
    self.trigger_medium_event(Key::ControlLeft, false).await?;

    Ok(())
  }

  pub async fn paste(&self) -> anyhow::Result<()> {
    // Trigger paste [Ctrl + V]

    self.trigger_medium_event(Key::ControlLeft, true).await?;
    self.trigger_long_event(Key::KeyV, true).await?;
    self.trigger_medium_event(Key::KeyV, false).await?;
    self.trigger_medium_event(Key::ControlLeft, false).await?;

    Ok(())
  }

  pub async fn select_all(&self) -> anyhow::Result<()> {
    // Trigger select all [Ctrl + A]

    self.trigger_medium_event(Key::ControlLeft, true).await?;
    self.trigger_medium_event(Key::KeyA, true).await?;
    self.trigger_medium_event(Key::KeyA, false).await?;
    self.trigger_medium_event(Key::ControlLeft, false).await?;

    Ok(())
  }

  pub async fn select_line(&self) -> anyhow::Result<()> {
    // Trigger move to line start [Home]

    self.trigger_medium_event(Key::Home, true).await?;
    self.trigger_medium_event(Key::Home, false).await?;

    // Trigger select and move to line end [Shift + End]

    self.trigger_medium_event(Key::ShiftLeft, true).await?;
    self.trigger_medium_event(Key::End, true).await?;
    self.trigger_medium_event(Key::End, false).await?;
    self.trigger_medium_event(Key::ShiftLeft, false).await?;

    Ok(())
  }

  pub async fn select_word(&self) -> anyhow::Result<()> {
    // Trigger move to word start [Control + LeftArrow]

    self.trigger_medium_event(Key::ControlLeft, true).await?;
    self.trigger_medium_event(Key::LeftArrow, true).await?;
    self.trigger_medium_event(Key::LeftArrow, false).await?;
    // Keep Control pressed

    // Trigger select and move to line end [Shift + RightArrow]

    self.trigger_medium_event(Key::ShiftLeft, true).await?;
    self.trigger_medium_event(Key::RightArrow, true).await?;
    self.trigger_medium_event(Key::RightArrow, false).await?;
    self.trigger_medium_event(Key::ShiftLeft, false).await?;

    // Release Control
    self.trigger_medium_event(Key::ControlLeft, false).await?;

    Ok(())
  }

  pub async fn select_chars(&self, count: usize) -> anyhow::Result<()> {
    self.select_chars_limited(count, 100usize).await
  }

  pub async fn select_chars_limited(&self, count: usize, limit: usize) -> anyhow::Result<()> {
    // Trigger select and move to left char [Shift + LeftArrow]

    if count == 0 { return Ok(()); }

    let count = count.min(limit);

    self.trigger_medium_event(Key::ShiftLeft, true).await?;

    for _ in 0..count {
      self.trigger_short_event(Key::LeftArrow, true).await?;
      self.trigger_short_event(Key::LeftArrow, false).await?;
    }

    self.trigger_medium_event(Key::ShiftLeft, false).await?;

    Ok(())
  }

  async fn trigger_long_event(&self, key: Key, is_press: bool) -> Result<(), SimulateError> {
    Self::trigger_event_ms(key, is_press, self.long_delay).await
  }

  async fn trigger_medium_event(&self, key: Key, is_press: bool) -> Result<(), SimulateError> {
    Self::trigger_event_ms(key, is_press, self.medium_delay).await
  }

  async fn trigger_short_event(&self, key: Key, is_press: bool) -> Result<(), SimulateError> {
    Self::trigger_event_ms(key, is_press, self.short_delay).await
  }

  async fn trigger_event_ms(key: Key, is_press: bool, delay: u64) -> Result<(), SimulateError> {
    let event = match is_press {
      true => EventType::KeyPress(key),
      false => EventType::KeyRelease(key),
    };

    simulate(&event)?;

    sleep(Duration::from_millis(delay)).await;

    Ok(())
  }
}
