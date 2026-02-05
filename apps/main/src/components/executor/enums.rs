use crate::common::structs::KeyboardSnapshot;

#[derive(Debug, Clone)]
pub enum InputType {
  Text(String),
  Events(Vec<KeyboardSnapshot>),
}
