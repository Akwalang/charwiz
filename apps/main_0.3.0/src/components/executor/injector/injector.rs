use std::usize;
use tokio::time::{sleep, Duration};

use rust_logger::*;

use crate::platform::Platform;
use crate::settings::Settings;

use crate::components::executor::commands;
use crate::components::executor::enums::InputType;
use crate::components::executor::emulator::Emulator;

use crate::common::enums::{InjectMethodEnum, UserInputCleanupEnum};
use crate::common::events::CommandEvent;
use crate::common::structs::KeyboardSnapshot;

type CharLine = (String, String, Vec<bool>);
type ExecuteLine = (String, String, String);

const PASTE_LINE_NAME: &'static str = "Paste";

pub struct Injector {
  platform: &'static Platform,
  settings: &'static Settings,
}

impl Injector {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings }
  }

  pub async fn inject(&self, emulator: &Emulator, event: &CommandEvent, input: InputType) -> anyhow::Result<()> {
    self.remove_injection_place(emulator, &event).await?;

    match input {
      InputType::Text(value) => {
        match event.injector.method {
          InjectMethodEnum::TypeAndPaste => self.use_type_and_paste(emulator, value).await?,
          InjectMethodEnum::TypeAndSkip => self.use_type_and_skip(emulator, value).await?,
          InjectMethodEnum::Emulate => self.use_type_and_skip(emulator, value).await?,
          InjectMethodEnum::Paste => self.use_paste(emulator, value).await?,
        }
      },
      InputType::Events(value) => self.use_emulate(emulator, &value).await?,
    }

    Ok(())
  }

  async fn remove_injection_place(&self, emulator: &Emulator, event: &CommandEvent) -> anyhow::Result<()> {
    debug!("<$>Injector</>: Clean: {:?}", event.injector.user_input_cleanup);

    match event.injector.user_input_cleanup {
      UserInputCleanupEnum::None => {},
      UserInputCleanupEnum::Backspace(count) => {
        for _ in 0..count {
          emulator.run(&commands::create_backspace_pipeline()).await?;
        }
      },
    }

    Ok(())
  }

  async fn use_emulate(&self, emulator: &Emulator, value: &[KeyboardSnapshot]) -> anyhow::Result<()> {
    let mut pipeline = Vec::with_capacity(1 + value.len());

    pipeline.extend_from_slice(value);
    pipeline.extend_from_slice(&commands::create_release_pipeline());

    emulator.run(&pipeline).await
  }

  async fn use_paste(&self, emulator: &Emulator, value: String) -> anyhow::Result<()> {
    let clipboard = &mut self.platform.clipboard.borrow_mut();

    clipboard.backup();
    clipboard.set_clipboard_text(&value)?;

    let clipboard_settings = self.settings.get_clipboard_hotkeys();
    let pipeline = commands::create_paste_pipeline(clipboard_settings.paste);

    let result = emulator.run(&pipeline).await;

    clipboard.restore();

    result
  }

  async fn use_type_and_paste(&self, emulator: &Emulator, value: String) -> anyhow::Result<()> {
    let execute_line = self.prepare_execute_line(value)?;

    for line in execute_line {
      if line.1 == PASTE_LINE_NAME {
        self.do_paste(emulator, line).await?;
      } else {
        self.do_type(emulator, line).await?;
      }
    }

    Ok(())
  }

  async fn use_type_and_skip(&self, emulator: &Emulator, value: String) -> anyhow::Result<()> {
    let execute_line = self.prepare_execute_line(value)?;

    for line in execute_line {
      if line.1 != PASTE_LINE_NAME {
        self.do_type(emulator, line).await?;
      }
    }

    Ok(())
  }

  async fn do_paste(&self, emulator: &Emulator, line: ExecuteLine) -> anyhow::Result<()> {
    self.platform.clipboard.borrow_mut().backup();
    self.platform.clipboard.borrow_mut().set_clipboard_text(&line.2)?;

    let clipboard = self.settings.get_clipboard_hotkeys();
    let pipeline = commands::create_paste_pipeline(clipboard.paste);

    emulator.run(&pipeline).await?;

    self.platform.clipboard.borrow_mut().restore();

    Ok(())
  }

  async fn do_type(&self, emulator: &Emulator, line: ExecuteLine) -> anyhow::Result<()> {
    let settings = self.settings.keyboard_layouts.borrow();
    let layout_chars = settings.items.get(&line.1).unwrap();

    self.platform.keyboard_layouts.borrow_mut().set_keyboard_layout(&line.0)?;

    sleep(Duration::from_millis(1)).await;

    let chars = line.2.chars();

    let mut pipeline: Vec<KeyboardSnapshot> = Vec::with_capacity(2 + chars.clone().count());

    pipeline.push(KeyboardSnapshot::default());

    for r#char in chars {
      let snapshot = layout_chars.chars.get(&r#char).unwrap();

      pipeline.push(snapshot.clone());
    }

    pipeline.push(KeyboardSnapshot::default());

    emulator.run(&pipeline).await?;

    Ok(())
  }

  fn prepare_execute_line(&self, value: String) -> anyhow::Result<Vec<ExecuteLine>> {
    self.settings.keyboard_layouts.borrow().check_keyboard_setup()?;

    let char_lines = self.calculate_char_lines(&value);

    Self::debug_print_char_lines(&char_lines);

    let execute_line = self.calculate_execute_line(&value, char_lines);

    Self::debug_print_execute_line(&execute_line);

    Ok(execute_line)
  }

  // Create lines of the following format
  // 00000409  en-US  1 1 1 1 1 1 1 0 0 0 0 0 0 1 0 0 0 0 0 0
  // 00000419  ru-RU  0 0 0 0 0 0 1 0 0 0 0 0 0 1 1 1 1 1 1 1
  //           Paste  0 0 0 0 0 0 0 1 1 1 1 1 1 0 0 0 0 0 0 0

  // TODO: Can panic if Settings::check_keyboard_setup wasn't run before
  fn calculate_char_lines(&self, value: &str) -> Vec<CharLine> {
    let settings_kl = self.settings.keyboard_layouts.borrow();
    let platform_kl = self.platform.keyboard_layouts.borrow();

    let capacity = platform_kl.items.len();
    let length = value.chars().count();

    let mut type_lines: Vec<CharLine> = Vec::with_capacity(1 + capacity); // +1 for paste line
    let mut paste_line: CharLine = (String::new(), String::from(PASTE_LINE_NAME), Vec::with_capacity(1 + length));

    for layout in &platform_kl.items {
      type_lines.push((layout.id.to_owned(), layout.name.to_owned(), Vec::with_capacity(length)));
    }

    for char in value.chars() {
      let mut is_any_exists = false;

      for layout in &platform_kl.items {
        let layout_name = layout.name.as_str();

        let setup = settings_kl.items.get(layout_name).unwrap();
        let is_exists = setup.chars.get(&char).is_some();

        is_any_exists = is_any_exists || is_exists;

        for line in &mut type_lines {
          if &line.1 != layout_name { continue; }

          line.2.push(is_exists);
          break;
        }
      }

      paste_line.2.push(!is_any_exists);
    }

    type_lines.push(paste_line);

    type_lines
  }

  // Covnver char lines into the following format
  // 00000409  en-US  "Native "
  //           Paste  "🔥🔥🔥🔥🔥🔥🔥"
  // 00000419  ru-RU  " скрипт"

  fn calculate_execute_line(&self, value: &str, char_lines: Vec<CharLine>) -> Vec<ExecuteLine> {
    let mut schedule: Vec<ExecuteLine> = vec![];

    let chars = value.chars().collect::<Vec<char>>();

    let mut offset: usize = 0;

    while offset < chars.len() {
      let mut longest_size: usize = 0;
      let mut longest_line_id: String = String::new();
      let mut longest_line_name: String = String::new();

      for (id, name, line) in &char_lines {
        let mut count = 0;

        for index in offset..chars.len() {
          if !line[index] { break; }
          count += 1;
        }

        if count > longest_size {
          longest_size = count;
          longest_line_id = id.to_owned();
          longest_line_name = name.to_owned();
        }
      }

      schedule.push((longest_line_id, longest_line_name, chars[offset..offset + longest_size].iter().collect()));

      offset += longest_size;
    }

    schedule
  }

  fn debug_print_char_lines(char_lines: &Vec<CharLine>) {
    debug!("<$>Injector</>: Type char lines:");

    for line in char_lines {
      let mut line_str = format!("  <i+>{}</>:", line.1);

      for is_exists in &line.2 {
        if *is_exists {
          line_str.push_str(" <i>1</>");
        } else {
          line_str.push_str(" <i>0</>");
        }
      }

      debug!("{}", line_str);
    }
  }

  fn debug_print_execute_line(execute_line: &Vec<ExecuteLine>) {
    debug!("<$>Injector</>: Type execute lines:");

    for line in execute_line {
      debug!("  <i+>{}</>: <i>{:?}</>", &line.1, &line.2);
    }
  }
}
