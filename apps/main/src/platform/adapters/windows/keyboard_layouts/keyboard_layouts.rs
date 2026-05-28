use std::time::Duration;

#[cfg(feature = "logger")]
use logger::*;

use crate::platform::common::structs::KeyboardLayoutItem;
use crate::utils::spin_lock::spin_sleep;

use super::utils;

pub struct KeyboardLayouts {
  pub items: Vec<KeyboardLayoutItem>,
}

impl KeyboardLayouts {
  pub fn new() -> Self {
    KeyboardLayouts { items: vec![] }
  }

  pub fn init(&mut self) {
    #[cfg(feature = "logger")]
    log!("<$>Windows::KeyboardLayouts</>: Init");

    for layout in utils::get_keyboard_layouts() {
      self.items.push(layout);
    }

    #[cfg(feature = "logger")]
    {
      let names = self.items.iter()
        .map(|lt| format!("<cyan>{} ({})</>", lt.name, lt.id))
        .collect::<Vec<String>>()
        .join(", ");

      log!("<$>Windows::KeyboardLayouts</>: Available keyboard layouts: {}", names);
    }
  }

  // pub fn get_keyboard_layouts(&self) -> &Vec<KeyboardLayoutItem> {
  //   &self.items
  // }

  pub fn get_first_layout(&self) -> KeyboardLayoutItem {
    if let Some(kbl) = self.items.first() {
      kbl.clone()
    } else {
      KeyboardLayoutItem::default()
    }
  }

  pub fn get_keyboard_layout_by_id(&self, id: &str) -> Option<&KeyboardLayoutItem> {
    self.items.iter().find(|lt| lt.id == id)
  }

  pub fn get_keyboard_layout_by_name(&self, name: &str) -> Option<&KeyboardLayoutItem> {
    self.items.iter().find(|lt| lt.name == name)
  }

  pub fn get_current_keyboard_layout(&self) -> anyhow::Result<&KeyboardLayoutItem> {
    let id = utils::get_current_keyboard_layout_id()?;

    Ok(&self.get_keyboard_layout_by_id(&id).unwrap())
  }

  // pub fn get_previous_keyboard_layout(&self) -> anyhow::Result<&KeyboardLayoutItem> {
  //   let id = utils::get_current_keyboard_layout_id()?;
    
  //   Ok(self.get_previous_to_keyboard_layout(&id))
  // }

  // pub fn get_next_keyboard_layout(&self) -> anyhow::Result<&KeyboardLayoutItem> {
  //   let id = utils::get_current_keyboard_layout_id()?;

  //   Ok(self.get_next_to_keyboard_layout(&id))
  // }

  pub fn get_previous_to_keyboard_layout(&self, id: &str) -> &KeyboardLayoutItem {
    let lts = &self.items;

    let idx: usize = lts.iter().position(|lt| lt.id == *id).or(Some(0)).unwrap();
    let next_idx = (lts.len() + idx - 1) % lts.len();

    &lts.get(next_idx).unwrap()
  }

  pub fn get_next_to_keyboard_layout(&self, id: &str) -> &KeyboardLayoutItem {
    let lts = &self.items;

    let idx: usize = lts.iter().position(|lt| lt.id == *id).or(Some(0)).unwrap();
    let next_idx = (lts.len() + idx + 1) % lts.len();

    &lts.get(next_idx).unwrap()
  }

  pub fn set_keyboard_layout(&self, layout_id: &str) -> anyhow::Result<Option<KeyboardLayoutItem>> {
    let Some(lt) = self.get_keyboard_layout_by_id(layout_id) else {
      #[cfg(feature = "logger")]
      warn!("<$>Platform::KeyboardLayouts</>: <->Keyboard layout not found: {}</>", layout_id);
      return Ok(None);
    };

    utils::switch_global_keyboard_layout(layout_id)?;

    #[cfg(feature = "logger")]
    log!("<$>Platform::KeyboardLayouts</>: Switch keyboard layout to: <i+>{}</> (<i+>{}</>)", lt.name, layout_id);

    spin_sleep(Duration::from_millis(5));

    Ok(Some(lt.clone()))
  }

  // pub fn set_previous_keyboard_layout(&self) -> anyhow::Result<Option<KeyboardLayoutItem>> {
  //   self.set_keyboard_layout(&self.get_previous_keyboard_layout()?.id)
  // }

  // pub fn set_next_keyboard_layout(&self) -> anyhow::Result<Option<KeyboardLayoutItem>> {
  //   self.set_keyboard_layout(&self.get_next_keyboard_layout()?.id)
  // }

  pub fn set_previous_to_keyboard_layout(&self, id: &str) -> anyhow::Result<Option<KeyboardLayoutItem>> {
    self.set_keyboard_layout(&self.get_previous_to_keyboard_layout(id).id)
  }

  pub fn set_next_to_keyboard_layout(&self, id: &str) -> anyhow::Result<Option<KeyboardLayoutItem>> {
    self.set_keyboard_layout(&self.get_next_to_keyboard_layout(id).id)
  }
}
