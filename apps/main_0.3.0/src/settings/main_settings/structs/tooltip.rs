use serde::Deserialize;

use crate::common::structs::{Executor, Injector};
use crate::common::enums::TooltipTypeEnum;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Tooltip {
  pub id: String,
  pub items: Vec<TooltipItem>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TooltipItem {
  pub settings: TooltipSettings,
  pub executor: Executor,
  pub injector: Injector,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TooltipSettings {
  pub r#type: TooltipTypeEnum,
  pub label: String,
}
