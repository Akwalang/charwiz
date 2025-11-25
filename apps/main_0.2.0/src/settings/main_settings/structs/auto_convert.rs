use serde::Deserialize;

use crate::settings::main_settings::structs::{Executor, Injector};

#[derive(Debug, Clone, Deserialize)]
pub struct AutoConvert {
  pub text: String,
  pub executor: Executor,
  pub injector: Injector,
}
