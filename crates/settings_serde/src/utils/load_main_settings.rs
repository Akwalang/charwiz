use settings_core::settings::main_settings::MainSettings;

use crate::settings::main_settings::MainSettingsRaw;

use crate::utils::read_json;

pub fn load_main_settings(path: &str) -> anyhow::Result<MainSettings> {
  let raw = read_json::<MainSettingsRaw>(path);

  if let Err(e) = raw {
    return Err(e);
  };

  Ok(raw.unwrap().into())
}
