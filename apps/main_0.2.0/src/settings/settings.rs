use std::collections::HashSet;

use rdev::Key;

pub struct Settings {
}

impl Settings {
  pub fn get_hotkeys() -> Vec<HashSet<Key>> {
    vec![
      HashSet::from([Key::ControlLeft, Key::ShiftLeft, Key::Alt]),
      HashSet::from([Key::ControlLeft, Key::ShiftLeft, Key::Alt, Key::KeyZ]),
    ]
  }

  pub fn get_commands() -> Vec<String> {
    vec![
      "cmd1".into(),
      "cmd2".into(),
      "cmd3".into(),
    ]
  }
}
