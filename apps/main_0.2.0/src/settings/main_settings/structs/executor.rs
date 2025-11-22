use serde::Deserialize;

use crate::common::enums::ExecutorType;

#[derive(Debug, Deserialize)]
pub struct Executor {
  #[serde(rename = "type")]
  pub r#type: ExecutorType,
  pub value: String,
}
