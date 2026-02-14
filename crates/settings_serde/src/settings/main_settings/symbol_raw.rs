use serde::Deserialize;

use settings_core::settings::main_settings::Symbol;

use crate::structs::{TransformerRaw, InjectorRaw, KeyboardSnapshotRaw};

#[derive(Deserialize)]
pub struct SymbolRaw {
  pub keys: KeyboardSnapshotRaw,
  pub transformer: TransformerRaw,
  pub injector: InjectorRaw,
}

impl Into<Symbol> for SymbolRaw {
  fn into(self) -> Symbol {
    Symbol {
      keys: self.keys.into(),
      transformer: self.transformer.into(),
      injector: self.injector.into(),
    }
  }
}
