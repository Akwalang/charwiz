use crate::structs::KeyboardSnapshot;

#[derive(Debug, Default, Clone)]
pub struct ClipboardHotkeys {
  pub copy: KeyboardSnapshot,
  pub paste: KeyboardSnapshot,
}
