use serde::Deserialize;

use settings_core::enums::ExecutorTypeEnum;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutorTypeRawEnum {
  Native,
  Plugin,
  Static,
}

impl Into<ExecutorTypeEnum> for ExecutorTypeRawEnum {
  fn into(self) -> ExecutorTypeEnum {
    match self {
      ExecutorTypeRawEnum::Native => ExecutorTypeEnum::Native,
      ExecutorTypeRawEnum::Plugin => ExecutorTypeEnum::Plugin,
      ExecutorTypeRawEnum::Static => ExecutorTypeEnum::Static,
    }
  }
}
