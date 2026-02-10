#[cfg(feature = "logger")]
use logger::*;

use settings_core::settings::platform_settings::{PlatformSettings as Settings};

use settings_serde::utils::load_platform_settings;

use crate::constants::PLATFORM_SETTINGS_FILE;

pub struct PlatformSettings {
  pub settings: Settings,
}

impl PlatformSettings {
  pub fn new() -> Self {
    Self { settings: Settings::default() }
  }

  pub fn init(&mut self) {
    #[cfg(feature = "logger")]
    log!("<$>Platform Settings</>: Init");

    self.settings = Self::load().unwrap_or_else(|_| panic!("Can't load platform settings"));
  }

  fn load() -> anyhow::Result<Settings> {
    #[cfg(feature = "logger")]
    log!("<$>Platform Settings</>: Loading settings: <i&>{}</>", PLATFORM_SETTINGS_FILE);

    let result = load_platform_settings(PLATFORM_SETTINGS_FILE);

    #[cfg(feature = "logger")]
    if let Err(ref e) = result {
      error!("<$>Platform Settings</>: Failed to load settings. Error: <i->{}</>", e.to_string());
    };

    result
  }
}
