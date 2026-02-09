use serde::Deserialize;

use settings_core::enums::TooltipTypeEnum;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TooltipTypeRawEnum {
  Option,
}

impl Into<TooltipTypeEnum> for TooltipTypeRawEnum {
  fn into(self) -> TooltipTypeEnum {
    match self {
      TooltipTypeRawEnum::Option => TooltipTypeEnum::Option,
    }
  }
}
