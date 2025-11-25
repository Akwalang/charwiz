use serde::Deserialize;

use crate::common::enums::InsertMethod;

use crate::settings::main_settings::structs::executor::Executor;

#[derive(Debug, Clone, Deserialize)]
pub struct AutoConvert {
  pub text: String,
  pub method: InsertMethod,
  pub layout: String,
  pub executor: Executor,
}
