use crate::common::structs::KeyboardSnapshot;

use crate::components::executor::commands;
use crate::components::executor::enums::InputType;

pub fn invert_case(target: &InputType) -> InputType {
  match target {
    InputType::Text(text) => invert_case_text(text),
    InputType::Events(events) => invert_case_events(events),
  }
}

fn invert_case_text(text: &String) -> InputType {
  let result: String = text.chars()
    .map(|c| {
      if c.is_uppercase() { return c.to_lowercase().to_string(); }
      if c.is_lowercase() { return c.to_uppercase().to_string(); }
      
      c.to_string()
    })
    .collect();

  InputType::Text(result)
}

fn invert_case_events(events: &[KeyboardSnapshot]) -> InputType {
  let mut result = Vec::with_capacity(1 + events.len());

  result.extend_from_slice(&commands::create_capslock_toggle_pipeline());
  result.extend_from_slice(&events);
  // result.extend_from_slice(&commands::create_capslock_toggle_pipeline());

  InputType::Events(result)
}
