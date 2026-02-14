use crate::enums::TooltipTypeEnum;
use crate::structs::{Transformer, Injector};

#[derive(Debug, Clone)]
pub struct Tooltip {
  pub id: String,
  pub items: Vec<TooltipItem>,
}

#[derive(Debug, Clone)]
pub struct TooltipItem {
  pub settings: TooltipSettings,
  pub transformer: Transformer,
  pub injector: Injector,
}

#[derive(Debug, Clone)]
pub struct TooltipSettings {
  pub r#type: TooltipTypeEnum,
  pub label: String,
}
