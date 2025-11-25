use crate::settings::structs::{Executor, Injector};

#[derive(Debug, Clone)]
pub struct CommandEvent {
  pub executor: Executor,
  pub injector: Injector,
}
