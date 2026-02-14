use crate::structs::{Transformer, Injector, KeyboardSnapshot};

#[derive(Debug, Clone)]
pub struct Symbol {
  pub keys: KeyboardSnapshot,
  pub transformer: Transformer,
  pub injector: Injector,
}
