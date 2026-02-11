#[cfg(feature = "logger")]
use logger::*;

use settings_core::settings::main_settings::{MainSettings as Settings};

use settings_serde::utils::load_main_settings;

use crate::constants::MAIN_SETTINGS_FILE;

pub struct MainSettings {
  pub settings: Settings,
}

impl MainSettings {
  pub fn new() -> Self {
    Self { settings: Settings::default() }
  }

  pub fn init(&mut self) {
    #[cfg(feature = "logger")]
    log!("<$>Main Settings</>: Init");

    self.settings = Self::load().unwrap_or_else(|_| panic!("Can't load main settings"));
  }

  fn load() -> anyhow::Result<Settings> {
    #[cfg(feature = "logger")]
    log!("<$>Main Settings</>: Loading settings: <i&>{}</>", MAIN_SETTINGS_FILE);

    let result = load_main_settings(MAIN_SETTINGS_FILE);

    #[cfg(feature = "logger")]
    if let Err(ref e) = result {
      error!("<$>Main Settings</>: Failed to load settings. Error: <i->{}</>", e.to_string());
    };

    result
  }
}
