use serde::Deserialize;

use crate::common::structs::{Executor, Injector};

use super::super::super::raw::structs::{ExecutorRaw, InjectorRaw};

#[derive(Deserialize)]
pub struct SwitchRaw {
  pub text: String,
  pub executor: ExecutorRaw,
  pub injector: InjectorRaw,
}

#[derive(Debug, Clone)]
pub struct Switch {
  pub text: Vec<char>,
  pub executor: Executor,
  pub injector: Injector,
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
