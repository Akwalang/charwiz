use std::error::Error;

use crate::services::platform::KeyboardLayout;

use crate::services::transform::native::Native;
use crate::services::transform::plugin::Plugin;

use crate::services::config::settings::SettingsAction;

pub struct Transform;

impl Transform {
  pub fn execute(
    value: String,
    action: &SettingsAction,
    kbl_before: &KeyboardLayout,
    kbl_after: &KeyboardLayout,
  ) -> Result<String, Box<dyn Error>> {
    if Native::is_applicable(action) {
      return Native::apply(&value, action, kbl_before, kbl_after);
    }

    // if Plugin::is_applicable(action) {
    //   return Plugin::apply(&value, action, kbl_before, kbl_after);
    // }

    Err(Box::<dyn Error>::from("Handler not found"))
  }
}
