use serde::Deserialize;

use settings_core::settings::main_settings::Command;

use crate::structs::{TransformerRaw, InjectorRaw};

#[derive(Deserialize)]
pub struct CommandRaw {
  pub cmd: String,
  pub transformer: TransformerRaw,
  pub injector: InjectorRaw,
}

impl Into<Command> for CommandRaw {
  fn into(self) -> Command {
    Command {
      cmd: self.cmd.chars().collect(),
      transformer: self.transformer.into(),
      injector: self.injector.into(),
    }
  }
}
