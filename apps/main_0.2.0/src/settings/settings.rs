use std::sync::{OnceLock, Mutex, MutexGuard};
use std::collections::HashSet;

use rust_logger::*;
use rdev::Key;

use crate::platform::Platform;
use crate::platform::common::structs::KeyboardLayoutItem;

use crate::settings::{MainSettings, KeyboardLayouts};

static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub struct Settings {
  pub keyboard_layouts: Mutex<KeyboardLayouts>,
  pub main_settings: Mutex<MainSettings>,
}

impl Settings {
  fn new() -> Self {
    Settings {
      keyboard_layouts: Mutex::new(KeyboardLayouts::new()),
      main_settings: Mutex::new(MainSettings::new()),
    }
  }

  fn get_instance() -> &'static Settings {
    SETTINGS.get_or_init(|| Settings::new())
  }

  pub fn init() {
    log!("<$>Settings</>: Init");

    {
      let mut settings_kl = Self::get_keyboard_layouts();
      let platform_kl = Platform::get_keyboard_layouts();

      settings_kl.init();

      for layout in &platform_kl.items {
        settings_kl.add(&layout.name);
      }
    }

    {
      let mut settings_main = Self::get_main_settings();
      
      settings_main.init();
    }
  }

  pub fn get_keyboard_layouts<'a>() -> MutexGuard<'a, KeyboardLayouts> {
    Self::get_instance().keyboard_layouts.lock().unwrap()
  }

  pub fn get_main_settings<'a>() -> MutexGuard<'a, MainSettings> {
    Self::get_instance().main_settings.lock().unwrap()
  }

  // test method
  pub fn get_hotkeys() -> Vec<HashSet<Key>> {
    vec![
      HashSet::from([Key::ControlLeft, Key::ShiftLeft, Key::Alt]),
      HashSet::from([Key::ControlLeft, Key::ShiftLeft, Key::Alt, Key::KeyZ]),
    ]
  }

  // test method
  pub fn get_commands() -> Vec<String> {
    vec![
      "cmd1".into(),
      "cmd2".into(),
      "cmd3".into(),
    ]
  }
}
