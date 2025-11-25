use serde::Deserialize;

use crate::common::enums::ExecutorTypeEnum;

#[derive(Debug, Clone, Deserialize)]
pub struct Executor {
  #[serde(rename = "type")]
  pub r#type: ExecutorTypeEnum,
  pub value: String,
}
