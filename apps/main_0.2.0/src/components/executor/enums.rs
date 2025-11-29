use crate::common::structs::KeyboardEventSnapshot;

#[derive(Debug)]
pub enum InputType {
  Text(String),
  Events(Box<Vec<KeyboardEventSnapshot>>),
}
