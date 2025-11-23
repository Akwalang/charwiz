use serde::Deserialize;

use crate::common::enums::InsertMethod;

#[derive(Debug, Clone, Deserialize)]
pub struct AutoConvert {
  pub text: String,
  pub method: InsertMethod,
  pub layout: String,
}
