use rdev::Key;

use rust_logger::*;

use crate::platform::common::structs::KeyboardLayoutItem;

use crate::common::structs::{KeyboardSnapshot, KeyboardModifiers};

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

  pub fn get_keyboard_layouts(&self) -> &Vec<KeyboardLayoutItem> {
    &self.items
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

  pub fn get_previous_keyboard_layout(&self) -> anyhow::Result<&KeyboardLayoutItem> {
    let id = utils::get_current_keyboard_layout_id()?;
    
    Ok(self.get_previous_to_keyboard_layout(&id))
  }

  pub fn get_next_keyboard_layout(&self) -> anyhow::Result<&KeyboardLayoutItem> {
    let id = utils::get_current_keyboard_layout_id()?;

    Ok(self.get_next_to_keyboard_layout(&id))
  }

  pub fn get_previous_to_keyboard_layout(&self, id: &str) -> &KeyboardLayoutItem {
    let lts = &self.items;

    let idx: usize = lts.iter().position(|lt| lt.id == *id).unwrap();
    let next_idx = (lts.len() + idx - 1) % lts.len();

    &lts.get(next_idx).unwrap()
  }

  pub fn get_next_to_keyboard_layout(&self, id: &str) -> &KeyboardLayoutItem {
    let lts = &self.items;

    let idx: usize = lts.iter().position(|lt| lt.id == *id).unwrap();
    let next_idx = (lts.len() + idx + 1) % lts.len();

    &lts.get(next_idx).unwrap()
  }

  pub fn set_keyboard_layout(&self, layout_id: &str) -> anyhow::Result<Option<KeyboardLayoutItem>> {
    let Some(lt) = self.get_keyboard_layout_by_id(layout_id) else {
      warn!("<$>Platform::KeyboardLayouts</>: <->Keyboard layout not found: {}</>", layout_id);
      return Ok(None);
    };

    utils::switch_global_keyboard_layout(layout_id)?;

    log!("<$>Platform::KeyboardLayouts</>: Switch keyboard layout to: <i+>{}</> (<i+>{}</>)", lt.name, layout_id);

    Ok(Some(lt.clone()))
  }

  pub fn set_previous_keyboard_layout(&self) -> anyhow::Result<Option<KeyboardLayoutItem>> {
    self.set_keyboard_layout(&self.get_previous_keyboard_layout()?.id)
  }

  pub fn set_next_keyboard_layout(&self) -> anyhow::Result<Option<KeyboardLayoutItem>> {
    self.set_keyboard_layout(&self.get_next_keyboard_layout()?.id)
  }

  pub fn set_previous_to_keyboard_layout(&self, id: &str) -> anyhow::Result<Option<KeyboardLayoutItem>> {
    self.set_keyboard_layout(&self.get_previous_to_keyboard_layout(id).id)
  }

  pub fn set_next_to_keyboard_layout(&self, id: &str) -> anyhow::Result<Option<KeyboardLayoutItem>> {
    self.set_keyboard_layout(&self.get_next_to_keyboard_layout(id).id)
  }

  pub fn is_banned_event_snapshots(snapshot: &KeyboardSnapshot) -> bool {
    let Some(key) = snapshot.key else {
      return false;
    };

    let modifiers = snapshot.modifiers;

    false
    || (key == Key::KeyL && modifiers.is_any_pressed_strict(KeyboardModifiers::META_ANY))
  }

  pub fn is_stack_breaker_snapshots(snapshot: &KeyboardSnapshot) -> bool {
    let Some(key) = snapshot.key else {
      return false;
    };

    let modifiers = snapshot.modifiers;

    false
    || key == Key::UpArrow
    || key == Key::DownArrow
    || key == Key::LeftArrow
    || key == Key::RightArrow
    || key == Key::PageUp
    || key == Key::PageDown
    || key == Key::Home
    || key == Key::End
    || (key == Key::Backspace && modifiers.is_any_pressed(KeyboardModifiers::CONTROL_ANY))
    || (key == Key::Tab && modifiers.is_any_pressed_strict(KeyboardModifiers::ALT_ANY))
    || (key == Key::KeyA && modifiers.is_any_pressed_strict(KeyboardModifiers::CONTROL_ANY))
    || (key == Key::KeyX && modifiers.is_any_pressed_strict(KeyboardModifiers::CONTROL_ANY))
    || (key == Key::KeyC && modifiers.is_any_pressed_strict(KeyboardModifiers::CONTROL_ANY))
    || (key == Key::KeyY && modifiers.is_any_pressed_strict(KeyboardModifiers::CONTROL_ANY))
    || (key == Key::KeyZ && modifiers.is_any_pressed_strict(KeyboardModifiers::CONTROL_ANY))
    || (key == Key::KeyZ && modifiers.is_any_pressed_strict(KeyboardModifiers::CONTROL_ANY | KeyboardModifiers::SHIFT_ANY))
  }
}
