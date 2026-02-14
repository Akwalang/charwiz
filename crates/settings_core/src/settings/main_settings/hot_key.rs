use crate::structs::{Transformer, Injector, KeyboardSnapshot};

#[derive(Debug, Clone)]
pub struct HotKey {
  pub keys: KeyboardSnapshot,
  pub transformer: Transformer,
  pub injector: Injector,
}
