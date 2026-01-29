use serde::Deserialize;

use crate::settings::structs_raw::KeyboardSnapshotRaw;

use crate::common::structs::{Executor, Injector, KeyboardSnapshot};

#[derive(Debug, Deserialize)]
pub struct HotkeyRaw {
  pub keys: KeyboardSnapshotRaw,
  pub executor: Executor,
  pub injector: Injector,
}

#[derive(Debug, Clone)]
pub struct HotKey {
  pub keys: KeyboardSnapshot,
  pub executor: Executor,
  pub injector: Injector,
}

impl Into<HotKey> for HotkeyRaw {
  fn into(self) -> HotKey {
    HotKey {
      keys: self.keys.into(),
      executor: self.executor,
      injector: self.injector,
    }
  }
}
