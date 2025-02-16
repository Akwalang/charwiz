use std::error::Error;

use crate::services::config::Config;
use crate::services::config::settings::{SettingsAction, SettingsKeyboardLayout};

use crate::services::platform::KeyboardLayout;

pub struct Native;

impl Native {
  pub fn is_applicable(action: &SettingsAction) -> bool {
    match action.handler.as_str() {
      "convert_char_layout" => true,
      "invert_case" => true,
      _ => false,
    }
  }

  pub fn apply(
    value: &str,
    action: &SettingsAction,
    kbl_before: &KeyboardLayout,
    kbl_after: &KeyboardLayout,
  ) -> Result<String, Box<dyn Error>> {
    match action.handler.as_str() {
      "convert_char_layout" => Ok(Self::convert_char_layout(value, kbl_before, kbl_after)),
      "invert_case" => Ok(Self::invert_case(value, kbl_before, kbl_after)),
      _ => Err(Box::<dyn Error>::from("Handler not found")),
    }
  }

  fn convert_char_layout(value: &str, before: &KeyboardLayout, after: &KeyboardLayout) -> String {
    let config: std::sync::MutexGuard<'_, Config> = Config::get_instance();

    let mapping = config.get_keyboard_layouts();

    let from = mapping.get(&before.name);
    let into = mapping.get(&after.name);

    if from.is_none() || into.is_none() {
      return value.to_string();
    }

    let from = from.unwrap();
    let into = into.unwrap();

    let mut result = String::with_capacity(value.len());

    for a in value.chars() {
      let b = Self::char_convert(a, from, into);

      result.push(b.or(Some(a)).unwrap());
    }

    result
  }

  fn char_convert(
    a: char,
    from: &SettingsKeyboardLayout,
    into: &SettingsKeyboardLayout,
  ) -> Option<char> {
    let idx = from.into_num.get(&a);

    if idx.is_none() { return None; }

    into.into_char.get(idx.unwrap()).copied()
  }

  fn invert_case(
    input: &str,
    _before: &KeyboardLayout,
    _after: &KeyboardLayout,
  ) -> String {
    input.chars()
      .map(|c| {
        if c.is_uppercase() {
          c.to_lowercase().to_string()
        } else if c.is_lowercase() {
          c.to_uppercase().to_string()
        } else {
          c.to_string()
        }
      })
      .collect()
  }
}
