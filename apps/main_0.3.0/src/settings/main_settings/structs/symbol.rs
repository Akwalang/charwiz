use serde::Deserialize;

use crate::common::structs::{Executor, Injector, KeyboardSnapshot};

use super::super::super::raw::structs::{ExecutorRaw, InjectorRaw, KeyboardSnapshotRaw};

#[derive(Deserialize)]
pub struct SymbolRaw {
  pub keys: KeyboardSnapshotRaw,
  pub executor: ExecutorRaw,
  pub injector: InjectorRaw,
}

#[derive(Debug, Clone)]
pub struct Symbol {
  pub keys: KeyboardSnapshot,
  pub executor: Executor,
  pub injector: Injector,
}

impl Into<Symbol> for SymbolRaw {
  fn into(self) -> Symbol {
    Symbol {
      keys: self.keys.into(),
      executor: self.executor.into(),
      injector: self.injector.into(),
    }
  }
}
