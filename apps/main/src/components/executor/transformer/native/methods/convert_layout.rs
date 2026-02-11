use crate::Platform;
use crate::Settings;

use crate::common::events::CommandEvent;
use crate::components::executor::enums::InputType;

pub fn convert_layout(
  _platform: &'static Platform,
  _settings: &'static Settings,
  _event: &CommandEvent,
  target: &InputType,
) -> InputType {
  let InputType::Text(text) = target else {
    return target.clone();
  };

  // let plat_layouts = platform.keyboard_layouts.borrow().get_keyboard_layouts();
  // let sets_layouts = settings.keyboard_layouts.borrow().get_keyboard_layouts();

  let chars = text.chars();
  let result = String::with_capacity(chars.clone().count() * 4); // cover utf-32

  // let layout = 1;

  // for r#char in chars.into_iter() {

  // }

  InputType::Text(result)
}
