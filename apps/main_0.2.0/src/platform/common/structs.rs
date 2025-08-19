#[derive(Debug, Clone)]
pub struct KeyboardLayoutItem {
  pub id: String,
  pub name: String,
}

impl KeyboardLayoutItem {
  pub fn new(id: String, name: String) -> Self {
    Self { id, name }
  }
}
