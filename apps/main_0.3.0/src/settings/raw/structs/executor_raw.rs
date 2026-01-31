use serde::Deserialize;

use crate::common::structs::Executor;

use super::super::enums::ExecutorTypeRawEnum;

#[derive(Deserialize)]
pub struct ExecutorRaw {
  #[serde(rename = "type")]
  pub r#type: ExecutorTypeRawEnum,
  pub value: String,
}

impl Into<Executor> for ExecutorRaw {
  fn into(self) -> Executor {
    Executor {
      r#type: self.r#type.into(),
      value: self.value,
    }
  }
}
