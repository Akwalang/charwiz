use serde::Deserialize;

use crate::common::enums::{
  UserInputCleanupEnum,
  KeyboardStateCleanupEnum,
  InjectMethodEnum,
  KeyboardLayoutEnum,
  TransformTargetEnum,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Injector {
  #[serde(default)]
  pub user_input_cleanup: UserInputCleanupEnum,
  #[serde(default)]
  pub keyboard_state_cleanup: KeyboardStateCleanupEnum,
  pub target: TransformTargetEnum,
  pub method: InjectMethodEnum,
  pub layout: KeyboardLayoutEnum,
}
