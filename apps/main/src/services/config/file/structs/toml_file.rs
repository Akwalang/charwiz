use std::collections::HashMap;
use serde::Deserialize;

use super::{TomlDelay, TomlAction};

#[derive(Debug, Deserialize)]
pub struct TomlFile {
  pub delay: TomlDelay,
  pub keyboard_layouts: HashMap<String, String>,
  pub actions: Vec<TomlAction>,
}
