use serde::Deserialize;

use crate::common::enums::TooltipTypeEnum;
use crate::settings::main_settings::structs::{Executor, Injector};

#[derive(Debug, Deserialize)]
pub struct Tooltip {
  pub id: String,
  pub items: Vec<TooltipItem>,
}

#[derive(Debug, Deserialize)]
pub struct TooltipItem {
  pub settings: TooltipSettings,
  pub executor: Executor,
  pub injector: Injector,
}

#[derive(Debug, Deserialize)]
pub struct TooltipSettings {
  pub r#type: TooltipTypeEnum,
  pub label: String,
}
