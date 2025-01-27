use std::sync::OnceLock;

use crate::constants::CONFIG_FILE;

use super::ConfigFile;

static CONFIG: OnceLock<ConfigFile> = OnceLock::new();

fn init_get_config() -> ConfigFile {
  let content = std::fs::read_to_string(CONFIG_FILE);

  if let Err(error) = content {
    panic!("Can't read config file.\nError: {:?}", error);
  }

  let config = toml::from_str(&content.unwrap());

  if let Err(error) = config {
    panic!("Can't parse config file.\nError: {:?}", error);
  }

  config.unwrap()
}

pub fn get_config() -> &'static ConfigFile {
  CONFIG.get_or_init(init_get_config)
}
