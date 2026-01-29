use serde::Deserialize;

use crate::common::structs::{Executor, Injector};

#[derive(Debug, Clone, Deserialize)]
pub struct AutoConvert {
  pub text: String,
  pub executor: Executor,
  pub injector: Injector,
}
