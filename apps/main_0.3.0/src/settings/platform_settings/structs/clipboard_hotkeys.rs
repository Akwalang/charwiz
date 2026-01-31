use serde::Deserialize;

use crate::common::structs::KeyboardSnapshot;
use crate::settings::raw::structs::KeyboardSnapshotRaw;

#[derive(Deserialize)]
pub struct ClipboardHotkeysRaw {
  copy: KeyboardSnapshotRaw,
  paste: KeyboardSnapshotRaw,
}

#[derive(Debug, Default, Clone)]
pub struct ClipboardHotkeys {
  pub copy: KeyboardSnapshot,
  pub paste: KeyboardSnapshot,
}

impl From<ClipboardHotkeysRaw> for ClipboardHotkeys {
  fn from(raw: ClipboardHotkeysRaw) -> Self {
    Self {
      copy: raw.copy.into(),
      paste: raw.paste.into(),
    }
  }
}
