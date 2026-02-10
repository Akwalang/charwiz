use crate::structs::{Executor, Injector, KeyboardSnapshot};

#[derive(Debug, Clone)]
pub struct HotKey {
  pub keys: KeyboardSnapshot,
  pub executor: Executor,
  pub injector: Injector,
}
