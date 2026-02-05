use serde::Deserialize;

use crate::common::enums::InjectMethodEnum;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InjectMethodRawEnum {
  Paste,
  Emulate,
  TypeAndSkip,
  TypeAndPaste,
}

impl Into<InjectMethodEnum> for InjectMethodRawEnum {
  fn into(self) -> InjectMethodEnum {
    match self {
      InjectMethodRawEnum::Paste        => InjectMethodEnum::Paste,
      InjectMethodRawEnum::Emulate      => InjectMethodEnum::Emulate,
      InjectMethodRawEnum::TypeAndSkip  => InjectMethodEnum::TypeAndSkip,
      InjectMethodRawEnum::TypeAndPaste => InjectMethodEnum::TypeAndPaste,
    }
  }
}