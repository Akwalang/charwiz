use serde::Deserialize;

use settings_core::structs::Injector;

use crate::enums::{
  UserInputCleanupRawEnum,
  KeyboardStateCleanupRawEnum,
  InjectMethodRawEnum,
  KeyboardLayoutRawEnum,
  TransformTargetRawEnum,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InjectorRaw {
  #[serde(default)]
  pub user_input_cleanup: UserInputCleanupRawEnum,
  #[serde(default)]
  pub keyboard_state_cleanup: KeyboardStateCleanupRawEnum,
  #[serde(default)]
  pub layout_before: KeyboardLayoutRawEnum,
  #[serde(default)]
  pub layout_after: KeyboardLayoutRawEnum,
  pub target: TransformTargetRawEnum,
  pub method: InjectMethodRawEnum,
}

impl Into<Injector> for InjectorRaw {
  fn into(self) -> Injector {
    Injector {
      user_input_cleanup: self.user_input_cleanup.into(),
      keyboard_state_cleanup: self.keyboard_state_cleanup.into(),
      layout_before: self.layout_before.into(),
      layout_after: self.layout_after.into(),
      target: self.target.into(),
      method: self.method.into(),
    }
  }
}
