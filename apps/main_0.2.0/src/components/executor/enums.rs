use crate::common::structs::KeyboardEventSnapshot;

#[derive(Debug, Clone)]
pub enum InputType {
  Text(String),
  Events(Vec<KeyboardEventSnapshot>),
}
