use std::collections::HashMap;
use std::sync::{OnceLock, Mutex, MutexGuard};

use crate::services::config::file::read_config_file;

use crate::services::config::settings::{
  Settings,
  SettingsDelay,
  SettingsAction,
  SettingsKeyboardLayout,
};

static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

fn init_config() -> Mutex<Config> {
  Mutex::new(Config::new())
}

pub struct Config {
  settings: Settings,
}

impl Config {
  pub fn get_instance() -> MutexGuard<'static, Config> {
    CONFIG.get_or_init(init_config).lock().unwrap()
  }

  pub fn new() -> Self {
    let settings = Self::load_settings();

    Self { settings }
  }

  fn load_settings() -> Settings {
    read_config_file().into()
  }

  pub fn reload_settings(&mut self) {
    self.settings = Self::load_settings();
  }

  pub fn get_delay(&self) -> &SettingsDelay {
    &self.settings.delay
  }

  pub fn get_keyboard_layouts(&self) -> &HashMap<String, SettingsKeyboardLayout> {
    &self.settings.keyboard_layouts
  }

  pub fn get_actions(&self) -> &Vec<SettingsAction> {
    &self.settings.actions
  }
}
