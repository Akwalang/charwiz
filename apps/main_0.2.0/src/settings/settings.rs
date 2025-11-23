use std::sync::{Mutex, MutexGuard};
use std::collections::HashSet;

use rust_logger::*;
use rdev::Key;

use crate::platform::Platform;
use crate::platform::common::structs::KeyboardLayoutItem;

use crate::settings::{MainSettings, KeyboardLayouts};
use crate::settings::main_settings::structs::{Command, Hotkey};

pub struct Settings {
  platform: &'static Platform,

  pub keyboard_layouts: Mutex<KeyboardLayouts>,
  pub main_settings: Mutex<MainSettings>,
}

impl Settings {
  pub fn new(platform: &'static Platform) -> &'static Self {
    Box::leak(Box::new(Settings {
      platform,
      keyboard_layouts: Mutex::new(KeyboardLayouts::new()),
      main_settings: Mutex::new(MainSettings::new()),
    }))
  }

  pub fn init(&self) {
    log!("<$>Settings</>: Init");

    {
      let mut settings_kl = self.get_keyboard_layouts();
      let platform_kl = self.platform.get_keyboard_layouts();

      settings_kl.init();

      for layout in &platform_kl.items {
        settings_kl.add(&layout.name);
      }
    }

    {
      self.get_main_settings().init();
    }
  }

  pub fn get_keyboard_layouts<'s>(&'s self) -> MutexGuard<'s, KeyboardLayouts> {
    self.keyboard_layouts.lock().unwrap()
  }

  pub fn get_main_settings<'s>(&'s self) -> MutexGuard<'s, MainSettings> {
    self.main_settings.lock().unwrap()
  }

  // test method
  pub fn get_hotkeys(&self) -> Vec<Hotkey> {
    let main = self.main_settings.lock().unwrap();

    main.settings.hotkeys.iter().cloned().collect()
  }

  // test method
  pub fn get_commands(&self) -> Vec<Command> {
    let main = self.main_settings.lock().unwrap();
    
    main.settings.commands.iter().cloned().collect()
  }
}
