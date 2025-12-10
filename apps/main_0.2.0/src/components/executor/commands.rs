use rdev::Key;

use crate::common::structs::{KeyboardEventSnapshot, KeyboardModifiers};

pub fn create_release_pipeline() -> [KeyboardEventSnapshot; 1] {
  [
    KeyboardEventSnapshot::default(),
  ]
}

pub fn create_copy_all_pipeline() -> [KeyboardEventSnapshot; 3] {
  [
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

pub fn create_copy_line_pipeline() -> [KeyboardEventSnapshot; 4] {
  [
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

pub fn create_copy_word_pipeline() -> [KeyboardEventSnapshot; 4] {
  [
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

pub fn create_copy_selection_pipeline() -> [KeyboardEventSnapshot; 2] {
  [
    KeyboardEventSnapshot::new(
      Some(Key::KeyC),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
    ),
    KeyboardEventSnapshot::default(),
  ]
}

pub fn create_paste_pipeline() -> [KeyboardEventSnapshot; 2] {
  [
    KeyboardEventSnapshot::new(
      Some(Key::KeyV),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
    ),
    KeyboardEventSnapshot::default(),
  ]
}

pub fn create_backspace_pipeline() -> [KeyboardEventSnapshot; 2] {
  [
    KeyboardEventSnapshot::new(
      Some(Key::Backspace),
      KeyboardModifiers::new(KeyboardModifiers::NONE),
    ),
    KeyboardEventSnapshot::default(),
  ]
}

pub fn create_capslock_toggle_pipeline() -> [KeyboardEventSnapshot; 2] {
  [
    KeyboardEventSnapshot::new(
      Some(Key::CapsLock),
      KeyboardModifiers::new(KeyboardModifiers::NONE),
    ),
    KeyboardEventSnapshot::default(),
  ]
}
