use crate::enums::TooltipTypeEnum;
use crate::structs::{Executor, Injector};

#[derive(Debug, Clone)]
pub struct Tooltip {
  pub id: String,
  pub items: Vec<TooltipItem>,
}

#[derive(Debug, Clone)]
pub struct TooltipItem {
  pub settings: TooltipSettings,
  pub executor: Executor,
  pub injector: Injector,
}

#[derive(Debug, Clone)]
pub struct TooltipSettings {
  pub r#type: TooltipTypeEnum,
  pub label: String,
}
