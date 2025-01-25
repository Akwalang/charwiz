use super::utils;

use crate::services::{KeyboardLayout, PlatformTrait};

pub struct Windows {
  current_keyboard_layout_id: String,
  keyboard_layouts: Vec<KeyboardLayout>,
}

impl Windows {
  pub fn new() -> Windows {
    let current_keyboard_layout_id = utils::get_current_keyboard_layout_id();
    let keyboard_layouts = utils::get_keyboard_layouts();

    println!("Current keyboard layout: {}", current_keyboard_layout_id);
    println!("Keyboard layouts: {:?}", keyboard_layouts);

    Windows { current_keyboard_layout_id, keyboard_layouts }
  }
}

impl PlatformTrait for Windows {
  fn get_keyboard_layout_by_id(&self, id: &str) -> Option<&KeyboardLayout> {
    let lts = &self.keyboard_layouts;

    lts.iter().find(|lt| lt.id == id)
  }

  fn get_current_keyboard_layout(&self) -> &KeyboardLayout {
    &self.get_keyboard_layout_by_id(&self.current_keyboard_layout_id).unwrap()
  }

  fn get_next_keyboard_layout(&self) -> &KeyboardLayout {
    let id = &self.current_keyboard_layout_id;
    let lts = &self.keyboard_layouts;

    let idx: usize = lts.iter().position(|lt| lt.id == *id).unwrap();
    let next_idx = (idx + 1usize) % lts.len();

    &lts.get(next_idx).unwrap()
  }

  fn set_keyboard_layouts(&mut self, layout_id: &str) -> Result<Option<KeyboardLayout>, Box<dyn std::error::Error + 'static>> {
    let lt = self.get_keyboard_layout_by_id(layout_id);

    if lt.is_none() { return Ok(None); }

    let lt = lt.unwrap().clone();

    utils::switch_global_keyboard_layout(layout_id)?;

    self.current_keyboard_layout_id = layout_id.to_string();

    println!("Switched to keyboard layout: {} ({})", lt.name, layout_id);

    Ok(Some(lt))
  }

  fn switch_keyboard_layout(&mut self) -> Result<Option<KeyboardLayout>, Box<dyn std::error::Error + 'static>> {
    let next_id = self.get_next_keyboard_layout().id.clone();

    self.set_keyboard_layouts(&next_id)
  }
}
