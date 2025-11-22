use std::sync::{Mutex, MutexGuard};

use rust_logger::*;

mod keyboard_layouts;
use keyboard_layouts::KeyboardLayouts;

pub struct Platform {
  pub keyboard_layouts: Mutex<KeyboardLayouts>,
}

impl Platform {
  pub fn new() -> &'static Self {
    let keyboard_layouts = KeyboardLayouts::new();

    Box::leak(Box::new(Platform {
      keyboard_layouts: Mutex::new(keyboard_layouts),
    }))
  }

  pub fn init(&self) {
    log!("<$>Windows</>: Init");

    let mut keyboard_layouts = self.keyboard_layouts.lock().unwrap();

    keyboard_layouts.init();
  }

  pub fn get_keyboard_layouts<'a>(&'a self) -> MutexGuard<'a, KeyboardLayouts> {
    self.keyboard_layouts.lock().unwrap()
  }
}
