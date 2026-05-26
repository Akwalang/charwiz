use crate::enums::{
  UserInputCleanupEnum,
  KeyboardStateCleanupEnum,
  InjectMethodEnum,
  KeyboardLayoutEnum,
  TransformTargetEnum,
};

#[derive(Debug, Clone)]
pub struct Injector {
  pub use_selected_context: bool,
  pub user_input_cleanup: UserInputCleanupEnum,
  pub keyboard_state_cleanup: KeyboardStateCleanupEnum,
  pub layout_before: KeyboardLayoutEnum,
  pub layout_after: KeyboardLayoutEnum,
  pub target: TransformTargetEnum,
  pub method: InjectMethodEnum,
}
