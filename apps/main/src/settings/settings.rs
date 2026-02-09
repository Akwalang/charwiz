use std::cell::RefCell;
use std::rc::Rc;

#[cfg(feature = "logger")]
use logger::*;

use crate::platform::Platform;

use crate::settings::main_settings::MainSettings;
use crate::settings::platform_settings::PlatformSettings;
use crate::settings::keyboard_layouts::KeyboardLayouts;

use crate::settings::main_settings::structs::{Switch, Command, HotKey, Symbol};
use crate::settings::platform_settings::structs::ClipboardHotkeys;

use crate::common::structs::KeyboardSnapshot;

pub struct Settings {
  platform: &'static Platform,

  pub keyboard_layouts: RefCell<KeyboardLayouts>,
  pub platform_settings: RefCell<PlatformSettings>,
  pub main_settings: RefCell<MainSettings>,
}

impl Settings {
  pub fn new(platform: &'static Platform) -> &'static Self {
    Box::leak(Box::new(Settings {
      platform,
      keyboard_layouts: RefCell::new(KeyboardLayouts::new(platform)),
      platform_settings: RefCell::new(PlatformSettings::new()),
      main_settings: RefCell::new(MainSettings::new()),
    }))
  }

  pub fn init(&self) {
    #[cfg(feature = "logger")]
    log!("<$>Settings</>: Init");

    {
      let mut settings_kl = self.keyboard_layouts.borrow_mut();
      let platform_kl = self.platform.keyboard_layouts.borrow();

      settings_kl.init();

      for layout in &platform_kl.items {
        settings_kl.add(&layout.name);
      }
    }

    self.main_settings.borrow_mut().init();
    self.platform_settings.borrow_mut().init();
  }

  pub fn get_switches(&self) -> Rc<Vec<Switch>> {
    let main = self.main_settings.borrow();

    main.settings.switches.clone()
  }

  pub fn get_commands(&self) -> Rc<Vec<Command>> {
    let main = self.main_settings.borrow();
    
    main.settings.commands.clone()
  }

  pub fn get_hotkeys(&self) -> Rc<Vec<HotKey>> {
    let main = self.main_settings.borrow();

    main.settings.hotkeys.clone()
  }

  pub fn get_symbols(&self) -> Rc<Vec<Symbol>> {
    let main = self.main_settings.borrow();

    main.settings.symbols.clone()
  }

  pub fn get_banned_hotkeys(&self) -> Rc<Vec<KeyboardSnapshot>> {
    let platform = self.platform_settings.borrow();

    platform.settings.banned_hotkeys.clone()
  }

  pub fn get_stack_brake_hotkeys(&self) -> Rc<Vec<KeyboardSnapshot>> {
    let platform = self.platform_settings.borrow();

    platform.settings.stack_brake_hotkeys.clone()
  }

  pub fn get_clipboard_hotkeys(&self) -> Rc<ClipboardHotkeys> {
    let platform = self.platform_settings.borrow();

    platform.settings.clipboard_hotkeys.clone()
  }

  pub fn get_switch_keyboard_layout_hotkeys(&self) -> Rc<Vec<KeyboardSnapshot>> {
    let platform = self.platform_settings.borrow();

    platform.settings.switch_keyboard_layout_hotkeys.clone()
  }
}
