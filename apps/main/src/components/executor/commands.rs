use rdev::Key;

use settings_core::structs::{KeyboardSnapshot, KeyboardModifiers};

pub fn create_release_pipeline() -> [KeyboardSnapshot; 1] {
  [
    KeyboardSnapshot::default(),
  ]
}

pub fn create_copy_all_pipeline(copy: KeyboardSnapshot) -> [KeyboardSnapshot; 3] {
  [
    KeyboardSnapshot::new(
      Some(Key::KeyA),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
    ),
    copy,
    KeyboardSnapshot::default(),
  ]
}

pub fn create_copy_line_pipeline(copy: KeyboardSnapshot) -> [KeyboardSnapshot; 4] {
  [
    KeyboardSnapshot::new(
      Some(Key::Home),
      KeyboardModifiers::new(KeyboardModifiers::NONE),
    ),
    KeyboardSnapshot::new(
      Some(Key::End),
      KeyboardModifiers::new(KeyboardModifiers::SHIFT_LEFT),
    ),
    copy,
    KeyboardSnapshot::default(),
  ]
}

pub fn create_copy_word_pipeline(copy: KeyboardSnapshot) -> [KeyboardSnapshot; 4] {
  [
    KeyboardSnapshot::new(
      Some(Key::LeftArrow),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT),
    ),
    KeyboardSnapshot::new(
      Some(Key::RightArrow),
      KeyboardModifiers::new(KeyboardModifiers::CONTROL_LEFT | KeyboardModifiers::SHIFT_LEFT),
    ),
    copy,
    KeyboardSnapshot::default(),
  ]
}

pub fn create_copy_selection_pipeline(copy: KeyboardSnapshot) -> [KeyboardSnapshot; 2] {
  [copy, KeyboardSnapshot::default()]
}

pub fn create_paste_pipeline(paste: KeyboardSnapshot) -> [KeyboardSnapshot; 2] {
  [paste, KeyboardSnapshot::default()]
}

pub fn create_backspace_pipeline() -> [KeyboardSnapshot; 1] {
  [
    KeyboardSnapshot::new(
      Some(Key::Backspace),
      KeyboardModifiers::new(KeyboardModifiers::NONE),
    ),
  ]
}

pub fn create_capslock_toggle_pipeline() -> [KeyboardSnapshot; 2] {
  [
    KeyboardSnapshot::new(
      Some(Key::CapsLock),
      KeyboardModifiers::new(KeyboardModifiers::NONE),
    ),
    KeyboardSnapshot::default(),
  ]
}
