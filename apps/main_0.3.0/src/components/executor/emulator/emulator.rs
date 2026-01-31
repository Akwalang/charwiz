use tokio::time::{sleep, Duration};
use rdev::{EventType, simulate};

use crate::settings::Settings;

use crate::common::structs::{KeyboardSnapshot, KeyboardModifiers};

pub struct Emulator {
  settings: &'static Settings,
}

impl Emulator {
  pub fn new(settings: &'static Settings) -> Self {
    Self { settings }
  }

  pub async fn run(&self, pipeline: &[KeyboardSnapshot]) -> anyhow::Result<()> {
    let queue = pipeline.iter();

    let mut current = &KeyboardSnapshot::default();
    let mut pipeline = Vec::<EventType>::with_capacity(1 + KeyboardSnapshot::CAPACITY); // +1 for before & after key field

    let delay = self.settings.main_settings.borrow().settings.timings.key_action_delay;

    sleep(Duration::from_micros(delay)).await;

    for next in queue {
      Self::put_snapshots_into_pipeline(&mut pipeline, current, next);

      for event in &pipeline {
        simulate(event)?;

        sleep(Duration::from_micros(delay)).await;
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
