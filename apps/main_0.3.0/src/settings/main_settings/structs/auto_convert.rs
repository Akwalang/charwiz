use serde::Deserialize;

use crate::common::structs::{Executor, Injector};

use super::super::super::raw::structs::{ExecutorRaw, InjectorRaw};

#[derive(Deserialize)]
pub struct AutoConvertRaw {
  pub text: String,
  pub executor: ExecutorRaw,
  pub injector: InjectorRaw,
}

#[derive(Debug, Clone)]
pub struct AutoConvert {
  pub text: String,
  pub executor: Executor,
  pub injector: Injector,
}

impl Into<AutoConvert> for AutoConvertRaw {
  fn into(self) -> AutoConvert {
    AutoConvert {
      text: self.text,
      executor: self.executor.into(),
      injector: self.injector.into(),
    }
  }
}