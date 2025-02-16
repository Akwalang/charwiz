use crate::constants::CONFIG_FILE;

use crate::services::config::file::TomlFile;

pub fn read_config_file() -> TomlFile {
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
