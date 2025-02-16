use super::KeyboardLayout;

pub trait PlatformTrait {
  fn get_instance() -> &'static Self;

  fn get_keyboard_layout_by_id(&self, id: &str) -> Option<&KeyboardLayout>;
  fn get_current_keyboard_layout(&self) -> &KeyboardLayout;
  fn get_next_keyboard_layout(&self) -> &KeyboardLayout;
  fn set_keyboard_layouts(&self, layout_id: &str) -> Result<Option<KeyboardLayout>, Box<dyn std::error::Error + 'static>>;
  fn switch_keyboard_layout(&self) -> Result<Option<KeyboardLayout>, Box<dyn std::error::Error + 'static>>;
}
