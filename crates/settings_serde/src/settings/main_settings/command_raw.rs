use serde::Deserialize;

use settings_core::settings::main_settings::Command;

use crate::structs::{ExecutorRaw, InjectorRaw};

#[derive(Deserialize)]
pub struct CommandRaw {
  pub cmd: String,
  pub executor: ExecutorRaw,
  pub injector: InjectorRaw,
}

impl Into<Command> for CommandRaw {
  fn into(self) -> Command {
    Command {
      cmd: self.cmd.chars().collect(),
      executor: self.executor.into(),
      injector: self.injector.into(),
    }
  }
}
