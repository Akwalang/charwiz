use crate::services::Settings;

pub struct Transformer {
  settings: Settings,
}

impl Transformer {
  pub fn new() -> Self {
    let settings = Settings::new();

    Self { settings }
  }
}
