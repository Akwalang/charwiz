use std::fs;
use std::path::Path;

use rust_logger::*;

use crate::constants::MAIN_SETTINGS_FILE;

use crate::settings::main_settings::structs::{SettingsRaw, Settings};

pub struct MainSettings {
  // hotkeys: Vec<Hotkey>,
}

impl MainSettings {
  pub fn new() -> Self {
    Self {}
  }

  pub fn init(&mut self) {
    log!("<$>Settings::MainSettings</>: Init");

    let Ok(sets) = Self::load() else {
      error!("<$>Settings::MainSettings</>: Can't load config json");
      panic!("Can't load config json");
    };
  }

  fn load() -> anyhow::Result<Settings> {
    let content = Self::read_file()?;

    let settings = Self::parse_json(content)?;
    let settings = Self::convert_json(settings);

    Ok(settings)
  }

  fn read_file() -> anyhow::Result<String> {
    let src = MAIN_SETTINGS_FILE;

    log!("<$>Settings::Custom</>: Loading custom settings: <i&>{}</>", src);

    let path = Path::new(&src);

    let content = fs::read_to_string(&path);

    if let Err(e) = content {
      error!("<$>Settings::Custom</>: Failed to load custom settings: <i&>{}</>", src);
      error!("<$>Settings::Custom</>: Error: <i&>{}</>", e.to_string());

      Err(anyhow::anyhow!(e.to_string()))
    } else {
      Ok(content.unwrap())
    }
  }

  fn parse_json(content: String) -> anyhow::Result<SettingsRaw> {
    let items = serde_json::from_str::<SettingsRaw>(&content);

    if let Err(e) = items {
      error!("<$>Settings::Custom</>: Invalid custom settings JSON: {}\n<i&>{}</>", e, content);
      return Err(anyhow::anyhow!(e.to_string()));
    }

    Ok(items.unwrap())
  }

  fn convert_json(items: SettingsRaw) -> Settings {
    Settings::from(items)
  }
}
