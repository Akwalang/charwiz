use crate::structs::{Executor, Injector, KeyboardSnapshot};

#[derive(Debug, Clone)]
pub struct Symbol {
  pub keys: KeyboardSnapshot,
  pub executor: Executor,
  pub injector: Injector,
}
