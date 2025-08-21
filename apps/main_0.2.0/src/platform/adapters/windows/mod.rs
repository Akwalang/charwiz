use std::sync::{OnceLock, Mutex};

use rust_logger::*;

mod keyboard_layouts;
use keyboard_layouts::KeyboardLayouts;

static PLATFORM: OnceLock<Platform> = OnceLock::new();

pub struct Platform {
  pub keyboard_layouts: Mutex<KeyboardLayouts>,
}

impl Platform {
  fn new() -> Self {
    let keyboard_layouts = KeyboardLayouts::new();

    Platform {
      keyboard_layouts: Mutex::new(keyboard_layouts),
    }
  }

  pub fn init() {
    log!("<purple>Windows</>: Init");

    let this = Self::get_instance();

    let mut keyboard_layouts = this.keyboard_layouts.lock().unwrap();

    keyboard_layouts.init();
  }

  pub fn get_instance() -> &'static Platform {
    PLATFORM.get_or_init(|| Platform::new())
  }

  pub fn get_keyboard_layouts<'a>() -> std::sync::MutexGuard<'a, KeyboardLayouts> {
    let this = Self::get_instance();
    
    this.keyboard_layouts.lock().unwrap()
  }
}
