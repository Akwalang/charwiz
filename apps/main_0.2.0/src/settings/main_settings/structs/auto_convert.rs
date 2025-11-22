use serde::Deserialize;

use crate::common::enums::InsertMethod;

#[derive(Debug, Deserialize)]
pub struct AutoConvert {
  text: String,
  method: InsertMethod,
  layout: String,
}
