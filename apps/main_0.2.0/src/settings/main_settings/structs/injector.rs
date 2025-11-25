use serde::Deserialize;

use crate::common::enums::{InjectMethodEnum, KeyboardLayoutEnum, TransformTargetEnum};

#[derive(Debug, Clone, Deserialize)]
pub struct Injector {
  pub target: TransformTargetEnum,
  pub method: InjectMethodEnum,
  pub layout: KeyboardLayoutEnum,
}
