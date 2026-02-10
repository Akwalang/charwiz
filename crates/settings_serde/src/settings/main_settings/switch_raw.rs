use serde::Deserialize;

use settings_core::settings::main_settings::Switch;

use crate::structs::{ExecutorRaw, InjectorRaw};

#[derive(Deserialize)]
pub struct SwitchRaw {
  pub text: String,
  pub executor: ExecutorRaw,
  pub injector: InjectorRaw,
}

impl Into<Switch> for SwitchRaw {
  fn into(self) -> Switch {
    Switch {
      text: self.text.chars().collect(),
      executor: self.executor.into(),
      injector: self.injector.into(),
    }
  }
}
