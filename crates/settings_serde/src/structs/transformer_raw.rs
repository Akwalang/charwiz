use serde::Deserialize;

use settings_core::structs::Transformer;

use crate::enums::TransformerTypeRawEnum;

#[derive(Deserialize)]
pub struct TransformerRaw {
  #[serde(rename = "type")]
  pub r#type: TransformerTypeRawEnum,
  pub value: String,
}

impl Into<Transformer> for TransformerRaw {
  fn into(self) -> Transformer {
    Transformer {
      r#type: self.r#type.into(),
      value: self.value,
    }
  }
}
