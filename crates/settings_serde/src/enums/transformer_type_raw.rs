use serde::Deserialize;

use settings_core::enums::TransformerTypeEnum;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransformerTypeRawEnum {
  Native,
  Plugin,
  Static,
}

impl Into<TransformerTypeEnum> for TransformerTypeRawEnum {
  fn into(self) -> TransformerTypeEnum {
    match self {
      TransformerTypeRawEnum::Native => TransformerTypeEnum::Native,
      TransformerTypeRawEnum::Plugin => TransformerTypeEnum::Plugin,
      TransformerTypeRawEnum::Static => TransformerTypeEnum::Static,
    }
  }
}
