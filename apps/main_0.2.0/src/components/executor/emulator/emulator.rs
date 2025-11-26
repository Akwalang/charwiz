use tokio::time::{sleep, Duration};
use rdev::{Key, EventType, simulate};

use crate::settings::Settings;

use crate::common::structs::{KeyboardEventSnapshot, KeyboardModifiers};

pub struct Emulator {
  settings: &'static Settings,
}

impl Emulator {
  pub fn new(settings: &'static Settings) -> Self {
    Self { settings }
  }

  pub async fn test(&self) -> anyhow::Result<()> {
    self.run(
      vec![
        KeyboardEventSnapshot { key: Some(Key::KeyH), modifiers: KeyboardModifiers(KeyboardModifiers::SHIFT_LEFT) },
        KeyboardEventSnapshot { key: Some(Key::KeyE), modifiers: KeyboardModifiers(KeyboardModifiers::NONE) },
        KeyboardEventSnapshot { key: Some(Key::KeyL), modifiers: KeyboardModifiers(KeyboardModifiers::NONE) },
        KeyboardEventSnapshot { key: Some(Key::KeyL), modifiers: KeyboardModifiers(KeyboardModifiers::NONE) },
        KeyboardEventSnapshot { key: Some(Key::KeyO), modifiers: KeyboardModifiers(KeyboardModifiers::NONE) },
        KeyboardEventSnapshot { key: Some(Key::Comma), modifiers: KeyboardModifiers(KeyboardModifiers::NONE) },
        KeyboardEventSnapshot { key: Some(Key::Space), modifiers: KeyboardModifiers(KeyboardModifiers::NONE) },
        KeyboardEventSnapshot { key: Some(Key::KeyW), modifiers: KeyboardModifiers(KeyboardModifiers::SHIFT_LEFT) },
        KeyboardEventSnapshot { key: Some(Key::KeyO), modifiers: KeyboardModifiers(KeyboardModifiers::NONE) },
        KeyboardEventSnapshot { key: Some(Key::KeyR), modifiers: KeyboardModifiers(KeyboardModifiers::NONE) },
        KeyboardEventSnapshot { key: Some(Key::KeyL), modifiers: KeyboardModifiers(KeyboardModifiers::NONE) },
        KeyboardEventSnapshot { key: Some(Key::KeyD), modifiers: KeyboardModifiers(KeyboardModifiers::NONE) },
        KeyboardEventSnapshot { key: Some(Key::Num1), modifiers: KeyboardModifiers(KeyboardModifiers::SHIFT_LEFT) },
        KeyboardEventSnapshot { key: None, modifiers: KeyboardModifiers(KeyboardModifiers::NONE) },
      ],
    ).await?;

    Ok(())
  }

  pub async fn run(&self, pipeline: Vec<KeyboardEventSnapshot>) -> anyhow::Result<()> {
    let queue = pipeline.iter();

    let mut current = &KeyboardEventSnapshot::default();

    let delay = self.settings.main_settings.lock().unwrap().settings.timings.keyActionDelay;

    for next in queue {
      let pipeline = Self::snapshots_to_pipeline(current, next);

      for event in pipeline {
        simulate(&event)?;

        sleep(Duration::from_nanos(delay)).await;
      }

      current = next;
    }

    Ok(())
  }

  fn snapshots_to_pipeline(before: &KeyboardEventSnapshot, after: &KeyboardEventSnapshot) -> Vec<EventType> {
    let release = KeyboardModifiers::compare_to_release(&before.modifiers, &after.modifiers);
    let press = KeyboardModifiers::compare_to_press(&before.modifiers, &after.modifiers);

    // +1 - because of field key can be in 2 snapshots
    let mut pipeline = Vec::<EventType>::with_capacity(1 + KeyboardEventSnapshot::CAPACITY);

    if before.key.is_some() {
      pipeline.push(EventType::KeyRelease(before.key.unwrap()));
    }

    for key in release {
      pipeline.push(EventType::KeyRelease(key));
    }

    for key in press {
      pipeline.push(EventType::KeyPress(key));
    }

    if after.key.is_some() {
      pipeline.push(EventType::KeyPress(after.key.unwrap()));
    }

    pipeline
  }
}
