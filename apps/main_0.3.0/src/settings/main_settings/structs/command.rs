use serde::Deserialize;

use crate::settings::main_settings::structs::{Executor, Injector};

#[derive(Debug, Clone, Deserialize)]
pub struct Command {
  pub cmd: String,
  pub executor: Executor,
  pub injector: Injector,
}
