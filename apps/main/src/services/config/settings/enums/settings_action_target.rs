#[derive(Debug, Clone)]
pub enum SettingsActionTarget {
  All,
  Line,
  Word,
  Selection,
  Clipboard,
  None,
}

impl Into<SettingsActionTarget> for String {
  fn into(self) -> SettingsActionTarget {
    match self.to_lowercase().as_str() {
      "all" => SettingsActionTarget::All,
      "line" => SettingsActionTarget::Line,
      "word" => SettingsActionTarget::Word,
      "selection" => SettingsActionTarget::Selection,
      "clipboard" => SettingsActionTarget::Clipboard,
      "none" => SettingsActionTarget::None,
      _ => SettingsActionTarget::None,
    }
  }
}
