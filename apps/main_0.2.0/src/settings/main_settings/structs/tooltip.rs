use serde::Deserialize;

use crate::common::enums::{TooltipType, TransformTarget};
use crate::settings::main_settings::structs::executor::Executor;

#[derive(Debug, Deserialize)]
pub struct Tooltip {
  pub id: String,
  pub items: Vec<TooltipItem>,
}

#[derive(Debug, Deserialize)]
pub struct TooltipItem {
  pub r#type: TooltipType,
  pub label: String,
  pub target: TransformTarget,
  pub executor: Executor,
}
