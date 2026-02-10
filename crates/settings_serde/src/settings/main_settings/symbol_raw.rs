use serde::Deserialize;

use settings_core::settings::main_settings::Symbol;

use crate::structs::{ExecutorRaw, InjectorRaw, KeyboardSnapshotRaw};

#[derive(Deserialize)]
pub struct SymbolRaw {
  pub keys: KeyboardSnapshotRaw,
  pub executor: ExecutorRaw,
  pub injector: InjectorRaw,
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
