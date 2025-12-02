use serde::Deserialize;

use crate::common::enums::{
  CleanupMethodEnum,
  InjectMethodEnum,
  KeyboardLayoutEnum,
  TransformTargetEnum,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Injector {
  pub target: TransformTargetEnum,
  pub cleanup: CleanupMethodEnum,
  pub method: InjectMethodEnum,
  pub layout: KeyboardLayoutEnum,
}
