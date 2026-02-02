use crate::platform::Platform;
use crate::settings::Settings;

use crate::common::events::CommandEvent;
use crate::common::enums::TransformTargetEnum;
use crate::common::structs::KeyboardSnapshot;

use super::super::commands;
use super::super::enums::InputType;
use super::super::emulator::Emulator;

pub struct Extractor {
  platform: &'static Platform,
  settings: &'static Settings,
}

impl Extractor {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings }
  }

  pub async fn extract(&self, emulator: &Emulator, event: &CommandEvent) -> anyhow::Result<InputType> {
    match event.injector.target {
      TransformTargetEnum::None => Self::extract_none(),
      TransformTargetEnum::All => self.extract_all(emulator).await,
      TransformTargetEnum::Line => self.extract_line(emulator).await,
      TransformTargetEnum::Word => self.extract_word(emulator).await,
      TransformTargetEnum::Input => self.extract_input(event),
      TransformTargetEnum::Events => self.extract_events(event),
      TransformTargetEnum::Command => Self::extract_command(),
      TransformTargetEnum::Selection => self.extract_selection(emulator).await,
      TransformTargetEnum::Clipboard => self.extract_clipboard(),
    }
  }

  async fn get_data_through_clipboard(&self, emulator: &Emulator, pipeline: &[KeyboardSnapshot]) -> anyhow::Result<InputType> {
    self.platform.clipboard.borrow_mut().backup();

    let _ = emulator.run(pipeline).await;

    let result = self.platform.clipboard.borrow().get_clipboard_text();

    self.platform.clipboard.borrow_mut().restore();

    if let Some(result) = result? {
      Ok(InputType::Text(result))
    } else {
      anyhow::bail!("Failed to extract all text from clipboard")
    }
  }

  fn extract_none() -> anyhow::Result<InputType> {
    Ok(InputType::Text(String::new()))
  }

  async fn extract_all(&self, emulator: &Emulator) -> anyhow::Result<InputType> {
    let clipboard_settings = self.settings.get_clipboard_hotkeys();

    self.get_data_through_clipboard(
      emulator,
      &commands::create_copy_all_pipeline(clipboard_settings.copy.clone()),
    ).await
  }

  async fn extract_line(&self, emulator: &Emulator) -> anyhow::Result<InputType> {
    let clipboard_settings = self.settings.get_clipboard_hotkeys();
    let pipeline = commands::create_copy_line_pipeline(clipboard_settings.copy.clone());

    self.get_data_through_clipboard(emulator, &pipeline).await
  }

  async fn extract_word(&self, emulator: &Emulator) -> anyhow::Result<InputType> {
    let clipboard_settings = self.settings.get_clipboard_hotkeys();
    let pipeline = commands::create_copy_word_pipeline(clipboard_settings.copy.clone());

    self.get_data_through_clipboard(emulator, &pipeline).await
  }

  fn extract_input(&self, event: &CommandEvent) -> anyhow::Result<InputType> {
    Ok(InputType::Text(event.char_stack.iter().collect()))
  }

  fn extract_events(&self, event: &CommandEvent) -> anyhow::Result<InputType> {
    Ok(InputType::Events(event.event_stack.clone()))
  }

  fn extract_command() -> anyhow::Result<InputType> {
    Ok(InputType::Text(String::new()))
  }

  async fn extract_selection(&self, emulator: &Emulator) -> anyhow::Result<InputType> {
    let clipboard_settings = self.settings.get_clipboard_hotkeys();
    let pipeline = commands::create_copy_selection_pipeline(clipboard_settings.copy.clone());

    self.get_data_through_clipboard(emulator, &pipeline).await
  }

  fn extract_clipboard(&self) -> anyhow::Result<InputType> {
    let result = self.platform.clipboard.borrow().get_clipboard_text();

    if let Some(result) = result? {
      Ok(InputType::Text(result))
    } else {
      anyhow::bail!("Failed to extract all text from clipboard")
    }
  }
}
