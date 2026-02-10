use logger::*;

use settings_core::settings::main_settings::Settings;
use settings_serde::settings::main_settings::SettingsRaw;

use super::super::utils;

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

    let raw = utils::read_json::<SettingsRaw>(MAIN_SETTINGS_FILE);

    if let Err(e) = raw {
      #[cfg(feature = "logger")]
      error!("<$>Main Settings</>: Failed to load settings. Error: <i->{}</>", e.to_string());
      return Err(e);
    };

    Ok(raw.unwrap().into())
  }
}
