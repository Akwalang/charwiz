use serde::Deserialize;

use settings_core::settings::main_settings::HotKey;

use crate::structs::{TransformerRaw, InjectorRaw, KeyboardSnapshotRaw};

#[derive(Deserialize)]
pub struct HotkeyRaw {
  pub keys: KeyboardSnapshotRaw,
  pub transformer: TransformerRaw,
  pub injector: InjectorRaw,
}

impl Into<HotKey> for HotkeyRaw {
  fn into(self) -> HotKey {
    HotKey {
      keys: self.keys.into(),
      transformer: self.transformer.into(),
      injector: self.injector.into(),
    }
  }
}
