use rdev::Key;

use crate::common::enums::InjectMethodEnum;
use crate::platform::Platform;

use crate::components::executor::emulator::Emulator;

use crate::common::events::CommandEvent;
use crate::common::structs::{KeyboardEventSnapshot, KeyboardModifiers};

pub struct Injector {
  platform: &'static Platform,
  emulator: Emulator,
}

impl Injector {
  pub fn new(platform: &'static Platform, emulator: Emulator) -> Self {
    Self { platform, emulator }
  }

  pub async fn inject(&self, event: CommandEvent, value: String) -> anyhow::Result<()> {
    if event.injector.method == InjectMethodEnum::Paste {
      self.paste(value).await?;
    }

    Ok(())
  }

  async fn paste(&self, value: String) -> anyhow::Result<()> {
    // TODO: Remove Mutex
    let _ = &self.platform.clipboard.lock().unwrap().backup();
    let _ = &self.platform.clipboard.lock().unwrap().set_clipboard_text(&value);

    let result = self.emulator.run(vec![
      KeyboardEventSnapshot::new(
        Some(Key::KeyV),
        KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
      ),
      KeyboardEventSnapshot::default(),
    ]).await;

    let _ = &self.platform.clipboard.lock().unwrap().restore();

    result
  }
}
