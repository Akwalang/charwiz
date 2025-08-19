use std::sync::OnceLock;

mod keyboard_layouts;
use keyboard_layouts::KeyboardLayouts;

static PLATFORM: OnceLock<Platform> = OnceLock::new();

pub struct Platform {
  pub keyboard_layouts: KeyboardLayouts,
}

impl Platform {
  fn new() -> Self {
    let keyboard_layouts = KeyboardLayouts::new();

    keyboard_layouts.print_items();

    Platform {
      keyboard_layouts,
    }
  }

  pub fn get_instance() -> &'static Platform {
    PLATFORM.get_or_init(|| Platform::new())
  }
}
