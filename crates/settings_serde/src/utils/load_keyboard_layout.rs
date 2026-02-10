use settings_core::settings::keyboard_layouts::KeyItem;

use crate::settings::keyboard_layouts::KeyItemRaw;

use super::read_json;

pub fn load_keyboard_layout(path: &str) -> anyhow::Result<Vec<KeyItem>> {
  let raw = read_json::<Vec<KeyItemRaw>>(path);
  
  if let Err(e) = raw {
    return Err(e);
  };

  let result = raw.unwrap_or(vec![]).into_iter().map(Into::into).collect();

  Ok(result)
}
