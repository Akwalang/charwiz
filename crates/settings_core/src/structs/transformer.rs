use crate::enums::TransformerTypeEnum;

#[derive(Debug, Clone)]
pub struct Transformer {
  pub r#type: TransformerTypeEnum,
  pub value: String,
}
