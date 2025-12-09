use rust_logger::*;

use crate::platform::common::structs::KeyboardLayoutItem;

use super::utils;

pub struct KeyboardLayouts {
  pub items: Vec<KeyboardLayoutItem>,
}

impl KeyboardLayouts {
  pub fn new() -> Self {
    KeyboardLayouts { items: vec![] }
  }

  pub fn init(&mut self) {
    log!("<$>Windows::KeyboardLayouts</>: Init");

    for layout in utils::get_keyboard_layouts() {
      self.items.push(layout);
    }

    let names = self.items.iter()
      .map(|lt| format!("<cyan>{} ({})</>", lt.name, lt.id))
      .collect::<Vec<String>>()
      .join(", ");

    log!("<$>Windows::KeyboardLayouts</>: Available keyboard layouts: {}", names);
  }

  pub fn get_keyboard_layout_by_id(&self, id: &str) -> Option<&KeyboardLayoutItem> {
    self.items.iter().find(|lt| lt.id == id)
  }

  pub fn get_current_keyboard_layout(&self) -> &KeyboardLayoutItem {
    let id = utils::get_current_keyboard_layout_id();

    &self.get_keyboard_layout_by_id(&id).unwrap()
  }

  pub fn get_next_keyboard_layout(&self) -> &KeyboardLayoutItem {
    let id = utils::get_current_keyboard_layout_id();
    let lts = &self.items;

    let idx: usize = lts.iter().position(|lt| lt.id == *id).unwrap();
    let next_idx = (idx + 1usize) % lts.len();

    &lts.get(next_idx).unwrap()
  }

  pub fn set_keyboard_layouts(&self, layout_id: &str) -> anyhow::Result<Option<KeyboardLayoutItem>> {
    let Some(lt) = self.get_keyboard_layout_by_id(layout_id) else {
      warn!("<$>Platform::KeyboardLayouts</>: <->Keyboard layout not found: {}</>", layout_id);
      return Ok(None);
    };

    utils::switch_global_keyboard_layout(layout_id)?;

    log!("<$>Platform::KeyboardLayouts</>: Switch keyboard layout to: <i+>{}</> (<i+>{}</>)", lt.name, layout_id);

    Ok(Some(lt.clone()))
  }

  pub fn switch_keyboard_layout(&self) -> anyhow::Result<Option<KeyboardLayoutItem>> {
    self.set_keyboard_layouts(&self.get_next_keyboard_layout().id)
  }
}
