use serde::Deserialize;

use crate::settings::main_settings::structs::executor::Executor;

#[derive(Debug, Deserialize)]
pub struct Command {
  pub cmd: String,
  pub method: String,
  pub executor: Executor,
}
