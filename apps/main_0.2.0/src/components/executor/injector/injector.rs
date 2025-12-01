use std::collections::HashMap;

use rdev::Key;

use crate::platform::Platform;
use crate::settings::Settings;

use crate::components::executor::emulator::Emulator;

use crate::common::enums::InjectMethodEnum;
use crate::common::events::CommandEvent;
use crate::common::structs::{KeyboardEventSnapshot, KeyboardModifiers};

pub struct Injector {
  platform: &'static Platform,
  settings: &'static Settings,
}

impl Injector {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings }
  }

  pub async fn inject(&self, emulator: &Emulator, event: CommandEvent, value: String) -> anyhow::Result<()> {
    match event.injector.method {
      InjectMethodEnum::TypeAndPaste => self.use_type_and_paste(value).await?,
      InjectMethodEnum::Paste => self.use_paste(emulator, value).await?,
      InjectMethodEnum::Type => self.use_type(value).await?,
    }

    Ok(())
  }

  async fn use_type_and_paste(&self, value: String) -> anyhow::Result<()> {
    self.settings.keyboard_layouts.borrow().check_keyboard_setup()?;

    let (type_lines, paste_line) = self.calculate_char_lines(&value);

    Ok(())
  }

  async fn use_paste(&self, emulator: &Emulator, value: String) -> anyhow::Result<()> {
    // TODO: Remove Mutex
    let _ = &self.platform.clipboard.borrow_mut().backup();
    let _ = &self.platform.clipboard.borrow_mut().set_clipboard_text(&value);

    let result = emulator.run(vec![
      Self::create_paste_snapshot(),
      KeyboardEventSnapshot::default(),
    ]).await;

    let _ = &self.platform.clipboard.borrow_mut().restore();

    result
  }

  async fn use_type(&self, value: String) -> anyhow::Result<()> {
    let settings_kl = self.settings.keyboard_layouts.borrow();

    settings_kl.check_keyboard_setup()?;


    let chars = value.chars();

    let mut pipeline: Vec<KeyboardEventSnapshot> = Vec::with_capacity(chars.clone().count());

    let platform_kl = self.platform.keyboard_layouts.borrow();

    let current_layout_name = platform_kl.get_current_keyboard_layout();
    let current_layout_chars = settings_kl.items.get(&current_layout_name.name).unwrap();

    for r#char in chars {
      let snapshot = current_layout_chars.chars.get(&r#char);

      if let Some(snapshot) = snapshot {
        pipeline.push(snapshot.clone());
        continue;
      }

      let mut next_layout_name: Option<String> = None;

      for item in &settings_kl.items {
        if item.1.chars.get(&r#char).is_none() { continue; }

        next_layout_name = Some(item.0.to_owned());
      }
    }

    Ok(())
  }

  fn create_paste_snapshot() -> KeyboardEventSnapshot {
    KeyboardEventSnapshot::new(
      Some(Key::KeyV),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
    )
  }

  // TODO: Can panic if Settings::check_keyboard_setup wasn't run before
  fn calculate_char_lines(&self, value: &str) -> (HashMap<String, Vec<bool>>, Vec<bool>) {
    let settings_kl = self.settings.keyboard_layouts.borrow();
    let platform_kl = self.platform.keyboard_layouts.borrow();

    let capacity = value.chars().count();

    let mut type_lines: HashMap<String, Vec<bool>> = HashMap::new();
    let mut paste_line: Vec<bool> = Vec::with_capacity(capacity);

    for layout in &platform_kl.items {
      type_lines.insert(layout.name.to_owned(), Vec::with_capacity(capacity));
    }

    for char in value.chars() {
      let mut is_any_exists = false;

      for layout in &platform_kl.items {
        let layout_name = layout.name.as_str();

        let setup = settings_kl.items.get(layout_name).unwrap();
        let is_exists = setup.chars.get(&char).is_some();

        is_any_exists = is_any_exists || is_exists;

        type_lines.get_mut(layout_name).unwrap().push(is_exists);
      }

      paste_line.push(!is_any_exists);
    }

    (type_lines, paste_line)
  }
}
