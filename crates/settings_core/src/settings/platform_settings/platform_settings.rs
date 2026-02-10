use std::rc::Rc;

use crate::structs::KeyboardSnapshot;

use super::ClipboardHotkeys;

#[derive(Debug, Default)]
pub struct PlatformSettings {
  pub clipboard_hotkeys: Rc<ClipboardHotkeys>,
  pub banned_hotkeys: Rc<Vec<KeyboardSnapshot>>,
  pub switch_keyboard_layout_hotkeys: Rc<Vec<KeyboardSnapshot>>,
  pub stack_brake_hotkeys: Rc<Vec<KeyboardSnapshot>>,
}
