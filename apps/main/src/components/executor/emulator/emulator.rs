use tokio::time::{sleep, Duration};
use rdev::{EventType, simulate};

use settings_core::structs::{KeyboardSnapshot, KeyboardModifiers};

use crate::settings::Settings;

pub struct Emulator {
  settings: &'static Settings,
}

impl Emulator {
  const CAPACITY: usize = 1 + KeyboardSnapshot::CAPACITY; // +1 for before & after key field

  pub fn new(settings: &'static Settings) -> Self {
    Self { settings }
  }

  pub async fn switch(&self, from: &KeyboardSnapshot, to: &KeyboardSnapshot) -> anyhow::Result<()> {
    sleep(Duration::from_millis(1)).await;

    let mut pipeline = Vec::<EventType>::with_capacity(Self::CAPACITY);

    Self::put_snapshots_into_pipeline(&mut pipeline, from, to);

    for event in &pipeline {
      simulate(event)?;

      // Must be awaited to not block event_hub
      sleep(Duration::from_millis(1)).await;
    }

    Ok(())
  }

  pub async fn run_pipeline(&self, pipeline: &[KeyboardSnapshot]) -> anyhow::Result<()> {
    let queue = pipeline.iter();

    let mut current = &KeyboardSnapshot::default();
    let mut pipeline = Vec::<EventType>::with_capacity(Self::CAPACITY);

    sleep(Duration::from_millis(1)).await;

    for next in queue {
      Self::put_snapshots_into_pipeline(&mut pipeline, current, next);

      for event in &pipeline {
        simulate(event)?;

        // Must be awaited to not block event_hub
        sleep(Duration::from_millis(1)).await;
      }

      current = next;
    }

    Ok(())
  }

  fn put_snapshots_into_pipeline(
    pipeline: &mut Vec<EventType>, // prevent reallocations
    before: &KeyboardSnapshot,
    after: &KeyboardSnapshot,
  ) {
    pipeline.clear();

    let release = KeyboardModifiers::compare_to_release(&before.modifiers, &after.modifiers);
    let press = KeyboardModifiers::compare_to_press(&before.modifiers, &after.modifiers);

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
  }
}
