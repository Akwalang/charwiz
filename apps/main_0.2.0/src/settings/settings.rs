use std::sync::{OnceLock, Mutex};
use std::collections::HashSet;

use rust_logger::*;

use rdev::Key;

use crate::platform::Platform;
use crate::platform::common::structs::KeyboardLayoutItem;

use crate::settings::KeyboardLayouts;

static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub struct Settings {
  pub keyboard_layouts: Mutex<KeyboardLayouts>,
}

impl Settings {
  fn new() -> Self {
    Settings {
      keyboard_layouts: Mutex::new(KeyboardLayouts::new()),
    }
  }

  fn get_instance() -> &'static Settings {
    SETTINGS.get_or_init(|| Settings::new())
  }

  pub fn init() {
    log!("<purple>Settings</>: Init");

    let this = Self::get_instance();
    let platform = Platform::get_instance();

    let mut keyboard_layouts = this.keyboard_layouts.lock().unwrap();

    keyboard_layouts.init();

    let items = &platform.keyboard_layouts.lock().unwrap().items;

    for layout in items {
      keyboard_layouts.add(&layout.name);
    }
  }

  pub fn get_keyboard_layouts<'a>() -> std::sync::MutexGuard<'a, KeyboardLayouts> {
    let this = Self::get_instance();
    
    this.keyboard_layouts.lock().unwrap()
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
