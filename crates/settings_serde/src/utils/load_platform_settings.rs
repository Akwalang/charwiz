use settings_core::settings::platform_settings::PlatformSettings;

use crate::settings::platform_settings::PlatformSettingsRaw;

use super::read_json;

pub fn load_platform_settings(path: &str) -> anyhow::Result<PlatformSettings> {
  let raw = read_json::<PlatformSettingsRaw>(path);

  if let Err(e) = raw {
    return Err(e);
  };

  Ok(raw.unwrap().into())
}
