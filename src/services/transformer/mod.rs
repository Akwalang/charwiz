use std::collections::HashSet;

use rdev::Key;

use crate::services::{Settings, HotKeyAction, PlatformAction};

pub struct Transformer {}

impl Transformer {
  pub fn new() -> Self {
    Self {}
  }

  pub fn convert(&self, keys: HashSet<Key>) -> Option<PlatformAction> {
    let hotkeys = Settings::get_hotkeys();

    let hotkey = hotkeys.iter().find(|hotkey| hotkey.is_matching(&keys));

    match hotkey {
      Some(hotkey) => Some(Self::hotkey_to_platform_action(&hotkey.action)),
      None => None,
    }
  }

  fn hotkey_to_platform_action(action: &HotKeyAction) -> PlatformAction {
    match action {
      HotKeyAction::ToggleLanguage => PlatformAction::ToggleLanguage,
    }
  }
}
