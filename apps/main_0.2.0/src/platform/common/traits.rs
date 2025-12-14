use super::KeyboardLayout;

pub trait PlatformTrait {
  fn get_instance() -> &'static Self;

  fn get_keyboard_layout_by_id(&self, id: &str) -> Option<&KeyboardLayout>;
  fn get_keyboard_layout_by_name(&self, name: &str) -> Option<&KeyboardLayout>;
  fn get_current_keyboard_layout(&self) -> &KeyboardLayout;
  fn get_next_keyboard_layout(&self) -> &KeyboardLayout;
  fn set_keyboard_layout(&self, layout_id: &str) -> anyhow::Result<Option<KeyboardLayout>>;
  fn set_previous_keyboard_layout(&self) -> anyhow::Result<Option<KeyboardLayout>>;
  fn set_next_keyboard_layout(&self) -> anyhow::Result<Option<KeyboardLayout>>;
  fn set_previous_to_keyboard_layout(&self, id: &str) -> anyhow::Result<Option<KeyboardLayout>>;
  fn set_next_to_keyboard_layout(&self, id: &str) -> anyhow::Result<Option<KeyboardLayout>>;
}
