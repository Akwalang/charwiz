use serde::Deserialize;

use settings_core::settings::main_settings::HotKey;

use crate::structs::{ExecutorRaw, InjectorRaw, KeyboardSnapshotRaw};

#[derive(Deserialize)]
pub struct HotkeyRaw {
  pub keys: KeyboardSnapshotRaw,
  pub executor: ExecutorRaw,
  pub injector: InjectorRaw,
}

impl Into<HotKey> for HotkeyRaw {
  fn into(self) -> HotKey {
    HotKey {
      keys: self.keys.into(),
      executor: self.executor.into(),
      injector: self.injector.into(),
    }
  }
}
