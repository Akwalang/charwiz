use rust_logger::*;
use rdev::Key;

use crate::platform::Platform;
use crate::settings::Settings;

use crate::components::executor::commands;
use crate::components::executor::emulator::Emulator;

use crate::common::enums::{InjectMethodEnum, CleanupMethodEnum};
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
    self.remove_injection_place(emulator, &event).await?;

    match event.injector.method {
      InjectMethodEnum::TypeAndPaste => self.use_type_and_paste(value).await?,
      InjectMethodEnum::TypeAndSkip => self.use_type_and_skip(value).await?,
      InjectMethodEnum::Paste => self.use_paste(emulator, value).await?,
    }

    Ok(())
  }

  async fn remove_injection_place(&self, emulator: &Emulator, event: &CommandEvent) -> anyhow::Result<()> {
    match event.injector.cleanup {
      CleanupMethodEnum::None => {},
      CleanupMethodEnum::Backspace => {
        // println!("{:#?}", event);

        for _ in 0..3 {
          emulator.run(commands::create_backspace_pipeline()).await?;
        }
      },
    }

    Ok(())
  }

  async fn use_paste(&self, emulator: &Emulator, value: String) -> anyhow::Result<()> {
    let clipboard = &mut self.platform.clipboard.borrow_mut();

    clipboard.backup();
    clipboard.set_clipboard_text(&value)?;

    let result = emulator.run(commands::create_paste_pipeline()).await;

    clipboard.restore();

    result
  }

  async fn use_type_and_paste(&self, value: String) -> anyhow::Result<()> {
    self.settings.keyboard_layouts.borrow().check_keyboard_setup()?;

    let (type_lines, paste_line) = self.calculate_char_lines(&value);

    Self::debug_print_char_lines(&type_lines, &paste_line);

    let execute_line = self.calculate_execute_line(&value, type_lines, paste_line);

    Self::debug_print_execute_line(&execute_line);

    Ok(())
  }

  async fn use_type_and_skip(&self, value: String) -> anyhow::Result<()> {
    self.settings.keyboard_layouts.borrow().check_keyboard_setup()?;

    let (type_lines, paste_line) = self.calculate_char_lines(&value);

    Self::debug_print_char_lines(&type_lines, &paste_line);

    Ok(())
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

  // TODO: Can panic if Settings::check_keyboard_setup wasn't run before
  fn calculate_char_lines(&self, value: &str) -> (Vec<(String, Vec<bool>)>, Vec<bool>) {
    let settings_kl = self.settings.keyboard_layouts.borrow();
    let platform_kl = self.platform.keyboard_layouts.borrow();

    let capacity = value.chars().count();

    let mut type_lines: Vec<(String, Vec<bool>)> = vec![];
    let mut paste_line: Vec<bool> = Vec::with_capacity(capacity);

    for layout in &platform_kl.items {
      type_lines.push((layout.name.to_owned(), Vec::with_capacity(capacity)));
    }

    for char in value.chars() {
      let mut is_any_exists = false;

      for layout in &platform_kl.items {
        let layout_name = layout.name.as_str();

        let setup = settings_kl.items.get(layout_name).unwrap();
        let is_exists = setup.chars.get(&char).is_some();

        is_any_exists = is_any_exists || is_exists;

        for line in &mut type_lines {
          if &line.0 != layout_name { continue; }

          line.1.push(is_exists);
          break;
        }
      }

      paste_line.push(!is_any_exists);
    }

    (type_lines, paste_line)
  }

  fn calculate_execute_line(&self, value: &str, type_lines: Vec<(String, Vec<bool>)>, paste_line: Vec<bool>) -> Vec<(String, String)> {
    let mut schedule: Vec<(String, String)> = vec![];

    let length = paste_line.len();

    let mut offset: usize = 0;

    let mut longest_size: usize = 0;
    let mut longest_line: String = String::new();

    let chars = value.chars().collect::<Vec<char>>();

    while offset < length {
      for (name, line) in &type_lines {
        let mut size = 0;

        for index in offset..length {
          if !line[index] { break; }
          size += 1;
        }

        if size > longest_size {
          longest_size = size;
          longest_line = name.to_owned();
        }
      }

      println!("Longest line: {} (size: {})", longest_line, longest_size);

      if longest_size == 0 {
        offset += 1;
        continue;
      }

      schedule.push((longest_line.clone(), chars[offset..offset + (longest_size - 1)].iter().collect()));
      offset += longest_size;
    }

    schedule
  }

  fn debug_print_char_lines(type_lines: &Vec<(String, Vec<bool>)>, paste_line: &Vec<bool>) {
    for line in type_lines {
      let mut line_str = format!("<i+>{}</>: ", line.0);

      for is_exists in &line.1 {
        if *is_exists {
          line_str.push_str("<i>1</> ");
        } else {
          line_str.push_str("<i>0</> ");
        }
      }

      log!("{}", line_str);
    }

    let mut paste_line_str = String::from("<i+>Paste</>: ");

    for is_exists in paste_line {
      if *is_exists {
        paste_line_str.push_str("<i>1</> ");
      } else {
        paste_line_str.push_str("<i>0</> ");
      }
    }

    log!("{}", paste_line_str);
  }

  fn debug_print_execute_line(execute_line: &Vec<(String, String)>) {
    println!("{:#?}", execute_line);
    // for line in execute_line {
    //   log!("<i+>{}</>: <i>{}</>", line.0, line.1);
    // }
  }
}
