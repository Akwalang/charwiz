use crate::services::KeyboardLayout;

pub fn invert_case(input: &str, _before: &KeyboardLayout, _after: &KeyboardLayout) -> String {
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
