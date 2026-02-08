use std::cell::RefCell;

use zero_cost_logger::*;

mod keyboard_layouts;
use keyboard_layouts::KeyboardLayouts;

mod clipboard;
use clipboard::Clipboard;

pub struct Platform {
  pub clipboard: RefCell<Clipboard>,
  pub keyboard_layouts: RefCell<KeyboardLayouts>,
}

impl Platform {
  pub fn new() -> &'static Self {
    let keyboard_layouts = KeyboardLayouts::new();

    Box::leak(Box::new(Platform {
      clipboard: RefCell::new(Clipboard::new()),
      keyboard_layouts: RefCell::new(keyboard_layouts),
    }))
  }

  pub fn init(&self) {
    log!("<$>Windows</>: Init");

    let mut keyboard_layouts = self.keyboard_layouts.borrow_mut();

    keyboard_layouts.init();
  }
}
