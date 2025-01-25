use std::collections::HashSet;

use rdev::Key;

use crate::services::{Settings, HotKeyAction, Command};

pub struct Hotkeys;

impl Hotkeys {
  pub fn convert(keys: HashSet<Key>) -> Option<Command> {
    let hotkeys = Settings::get_hotkeys();

    let hotkey = hotkeys.iter().find(|hotkey| hotkey.is_matching(&keys));

    match hotkey {
      Some(hotkey) => Some(Self::hotkey_to_platform_action(&hotkey.action)),
      None => None,
    }
  }

  fn hotkey_to_platform_action(action: &HotKeyAction) -> Command {
    match action {
      HotKeyAction::SwitchLanguage => Command::SwitchLanguage,
    }
  }
}
