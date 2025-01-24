use super::{KeyboardLayout, PlatformAction};

pub trait Platform {
  fn get_keyboard_layout_by_id(&self, id: &str) -> Option<&KeyboardLayout>;
  fn get_current_keyboard_layout(&self) -> &KeyboardLayout;
  fn get_next_keyboard_layout(&self) -> &KeyboardLayout;
  fn set_keyboard_layouts(&mut self, layout_id: &str) -> Result<Option<KeyboardLayout>, Box<dyn std::error::Error + 'static>>;
  fn switch_keyboard_layout(&mut self) -> Result<Option<KeyboardLayout>, Box<dyn std::error::Error + 'static>>;
  fn apply_output(&mut self, output: PlatformAction);
}
