use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TomlAction {
  pub keys: Vec<String>,
  pub target: String,
  #[serde(default)]
  pub switch_keyboard_layout: bool,
  #[serde(default)]
  pub keep_selection: bool,
  pub handler: String,
}
