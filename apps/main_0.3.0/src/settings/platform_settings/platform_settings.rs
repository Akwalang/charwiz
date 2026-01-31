use rust_logger::*;

use super::structs::{SettingsRaw, Settings};

use crate::utils;
use crate::constants::PLATFORM_SETTINGS_FILE;

pub struct PlatformSettings {
  pub settings: Settings,
}

impl PlatformSettings {
  pub fn new() -> Self {
    Self { settings: Settings::default() }
  }

  pub fn init(&mut self) {
    log!("<$>Platform Settings</>: Init");

    self.settings = Self::load().unwrap_or_else(|_| panic!("Can't load platform settings"));
  }

  fn load() -> anyhow::Result<Settings> {
    log!("<$>Platform Settings</>: Loading settings: <i&>{}</>", PLATFORM_SETTINGS_FILE);

    let raw = utils::read_json::<SettingsRaw>(PLATFORM_SETTINGS_FILE);

    if let Err(e) = raw {
      error!("<$>Platform Settings</>: Failed to load settings. Error: <i->{}</>", e.to_string());
      return Err(e);
    };

    Ok(raw.unwrap().into())
  }
}
