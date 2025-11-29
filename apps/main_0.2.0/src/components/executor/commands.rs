use rdev::Key;

use crate::common::structs::{KeyboardEventSnapshot, KeyboardModifiers};

pub fn create_copy_all_pipeline() -> Vec<KeyboardEventSnapshot> {
  vec![
    KeyboardEventSnapshot::new(
      Some(Key::KeyA),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
    ),
    KeyboardEventSnapshot::new(
      Some(Key::KeyC),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
    ),
    KeyboardEventSnapshot::default(),
  ]
}

pub fn create_copy_line_pipeline() -> Vec<KeyboardEventSnapshot> {
  vec![
    KeyboardEventSnapshot::new(
      Some(Key::Home),
      KeyboardModifiers::new(KeyboardModifiers::NONE),
    ),
    KeyboardEventSnapshot::new(
      Some(Key::End),
      KeyboardModifiers::new(KeyboardModifiers::SHIFT_LEFT),
    ),
    KeyboardEventSnapshot::new(
      Some(Key::KeyC),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
    ),
    KeyboardEventSnapshot::default(),
  ]
}

pub fn create_copy_word_pipeline() -> Vec<KeyboardEventSnapshot> {
  vec![
    KeyboardEventSnapshot::new(
      Some(Key::LeftArrow),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
    ),
    KeyboardEventSnapshot::new(
      Some(Key::RightArrow),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT | KeyboardModifiers::SHIFT_LEFT),
    ),
    KeyboardEventSnapshot::new(
      Some(Key::KeyC),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
    ),
    KeyboardEventSnapshot::default(),
  ]
}

pub fn create_copy_selection_pipeline() -> Vec<KeyboardEventSnapshot> {
  vec![
    KeyboardEventSnapshot::new(
      Some(Key::KeyC),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
    ),
    KeyboardEventSnapshot::default(),
  ]
}
