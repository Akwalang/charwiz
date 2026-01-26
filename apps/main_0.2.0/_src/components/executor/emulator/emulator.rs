use tokio::time::{sleep, Duration};
use rdev::{EventType, simulate};

use crate::settings::Settings;

use crate::common::structs::{KeyboardEventSnapshot, KeyboardModifiers};

pub struct Emulator {
  settings: &'static Settings,
}

impl Emulator {
  pub fn new(settings: &'static Settings) -> Self {
    Self { settings }
  }

  pub async fn run(&self, pipeline: &[KeyboardEventSnapshot]) -> anyhow::Result<()> {
    let queue = pipeline.iter();

    let mut current = &KeyboardEventSnapshot::default();

    let delay = self.settings.main_settings.borrow().settings.timings.key_action_delay;

    // +1 - because of field key can be some in 2 snapshots
    let mut pipeline = Vec::<EventType>::with_capacity(1 + KeyboardEventSnapshot::CAPACITY);

    sleep(Duration::from_nanos(delay)).await;

    for next in queue {
      Self::put_snapshots_into_pipeline(&mut pipeline, current, next);

      for event in &pipeline {
        simulate(event)?;

        sleep(Duration::from_nanos(delay)).await;
      }

      current = next;
    }

    Ok(())
  }

  fn put_snapshots_into_pipeline(
    pipeline: &mut Vec<EventType>, // prevent reallocations
    before: &KeyboardEventSnapshot,
    after: &KeyboardEventSnapshot,
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
