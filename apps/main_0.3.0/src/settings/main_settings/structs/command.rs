use serde::Deserialize;

use crate::common::structs::{Executor, Injector};

use super::super::super::raw::structs::{ExecutorRaw, InjectorRaw};

#[derive(Deserialize)]
pub struct CommandRaw {
  pub cmd: String,
  pub executor: ExecutorRaw,
  pub injector: InjectorRaw,
}

#[derive(Debug, Clone)]
pub struct Command {
  pub cmd: String,
  pub executor: Executor,
  pub injector: Injector,
}

impl Into<Command> for CommandRaw {
  fn into(self) -> Command {
    Command {
      cmd: self.cmd,
      executor: self.executor.into(),
      injector: self.injector.into(),
    }
  }
}
