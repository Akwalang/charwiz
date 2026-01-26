use std::cell::RefCell;

use rust_logger::*;

use crate::platform::Platform;

use crate::settings::{MainSettings, KeyboardLayouts};
use crate::settings::main_settings::structs::{AutoConvert, Command, HotKey};

pub struct Settings {
  platform: &'static Platform,

  pub keyboard_layouts: RefCell<KeyboardLayouts>,
  pub main_settings: RefCell<MainSettings>,
}

impl Settings {
  pub fn new(platform: &'static Platform) -> &'static Self {
    Box::leak(Box::new(Settings {
      platform,
      keyboard_layouts: RefCell::new(KeyboardLayouts::new(platform)),
      main_settings: RefCell::new(MainSettings::new()),
    }))
  }

  pub fn init(&self) {
    log!("<$>Settings</>: Init");

    {
      let mut settings_kl = self.keyboard_layouts.borrow_mut();
      let platform_kl = self.platform.keyboard_layouts.borrow();

      settings_kl.init();

      for layout in &platform_kl.items {
        settings_kl.add(&layout.name);
      }
    }

    {
      self.main_settings.borrow_mut().init();
    }
  }

  // TODO: optimize
  pub fn get_auto_converters(&self) -> Vec<AutoConvert> {
    let main = self.main_settings.borrow();

    main.settings.auto_converts.clone()
  }

  // TODO: optimize
  pub fn get_commands(&self) -> Vec<Command> {
    let main = self.main_settings.borrow();
    
    main.settings.commands.clone()
  }

  // TODO: optimize
  pub fn get_hotkeys(&self) -> Vec<HotKey> {
    let main = self.main_settings.borrow();

    main.settings.hotkeys.clone()
  }
}
