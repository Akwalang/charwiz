use super::utils;

use crate::services::{KeyboardLayout, PlatformTrait};

pub struct Windows {
  keyboard_layouts: Vec<KeyboardLayout>,
}

impl Windows {
  pub fn new() -> Windows {
    let keyboard_layouts = utils::get_keyboard_layouts();

    Windows { keyboard_layouts }
  }

  pub fn log_state(&self) {
    let id = utils::get_current_keyboard_layout_id();

    println!("Keyboard layouts:");

    for lt in &self.keyboard_layouts {
      let marker = if lt.id == id { "+" } else { " " };

      println!(" {} {} ({})", marker, lt.name, lt.id);
    }
  }
}

impl PlatformTrait for Windows {
  fn get_keyboard_layout_by_id(&self, id: &str) -> Option<&KeyboardLayout> {
    let lts = &self.keyboard_layouts;

    lts.iter().find(|lt| lt.id == id)
  }

  fn get_current_keyboard_layout(&self) -> &KeyboardLayout {
    let id = utils::get_current_keyboard_layout_id();

    &self.get_keyboard_layout_by_id(&id).unwrap()
  }

  fn get_next_keyboard_layout(&self) -> &KeyboardLayout {
    let id = utils::get_current_keyboard_layout_id();
    let lts = &self.keyboard_layouts;

    let idx: usize = lts.iter().position(|lt| lt.id == *id).unwrap();
    let next_idx = (idx + 1usize) % lts.len();

    &lts.get(next_idx).unwrap()
  }

  fn set_keyboard_layouts(&self, layout_id: &str) -> Result<Option<KeyboardLayout>, Box<dyn std::error::Error + 'static>> {
    let lt = self.get_keyboard_layout_by_id(layout_id);

    if lt.is_none() { return Ok(None); }

    let lt = lt.unwrap().clone();

    utils::switch_global_keyboard_layout(layout_id)?;

    println!(" + {} ({})", lt.name, layout_id);

    Ok(Some(lt))
  }

  fn switch_keyboard_layout(&self) -> Result<Option<KeyboardLayout>, Box<dyn std::error::Error + 'static>> {
    let next_id = self.get_next_keyboard_layout().id.clone();

    self.set_keyboard_layouts(&next_id)
  }
}
