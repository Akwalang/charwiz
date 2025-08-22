use std::collections::HashMap;

use rust_logger::*;
use rdev::Key;

use crate::common::structs::KeyboardModifiers;
use crate::settings::keyboard_layouts::LayoutItem;

use super::KeyInsert;

pub struct KeyboardLayouts {
  items: HashMap<String, LayoutItem>,
}

impl KeyboardLayouts {
  pub fn new() -> Self {
    KeyboardLayouts {
      items: HashMap::new(),
    }
  }

  pub fn init(&mut self) {
    log!("<$>Settings::KeyboardLayouts</>: Init");
  }

  pub fn add(&mut self, name: &str) {
    let layout = LayoutItem::new(name);

    self.items.insert(name.to_owned(), layout);
  }

  pub fn find_combination(&self, layout_name: &str, key: &Key, modifiers: KeyboardModifiers) -> (Option<char>, bool) {
    let mut char = None;
    let mut is_exists = false;

    let compare = |item: &&KeyInsert| item.modifiers == modifiers;

    for (name, layout_item) in &self.items {
      let Some(setup) = layout_item.keys.get(key) else { continue; };
      let Some(insert) = setup.insert.iter().find(compare) else { continue; };

      is_exists = true;

      if name == layout_name {
        char = Some(insert.r#char);
        break;
      }
    }

    (char, is_exists)
  }
}
