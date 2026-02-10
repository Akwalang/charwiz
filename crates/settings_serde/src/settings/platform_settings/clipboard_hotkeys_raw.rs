use serde::Deserialize;

use settings_core::settings::platform_settings::ClipboardHotkeys;

use crate::structs::KeyboardSnapshotRaw;

#[derive(Deserialize)]
pub struct ClipboardHotkeysRaw {
  copy: KeyboardSnapshotRaw,
  paste: KeyboardSnapshotRaw,
}

impl Into<ClipboardHotkeys> for ClipboardHotkeysRaw {
  fn into(self) -> ClipboardHotkeys {
    ClipboardHotkeys {
      copy: self.copy.into(),
      paste: self.paste.into(),
    }
  }
}
