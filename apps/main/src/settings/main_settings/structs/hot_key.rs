use serde::Deserialize;

use crate::common::structs::{Executor, Injector, KeyboardSnapshot};

use super::super::super::raw::structs::{ExecutorRaw, InjectorRaw, KeyboardSnapshotRaw};

#[derive(Deserialize)]
pub struct HotkeyRaw {
  pub keys: KeyboardSnapshotRaw,
  pub executor: ExecutorRaw,
  pub injector: InjectorRaw,
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
      executor: self.executor.into(),
      injector: self.injector.into(),
    }
  }
}
