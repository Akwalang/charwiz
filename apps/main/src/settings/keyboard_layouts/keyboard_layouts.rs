use std::collections::HashMap;

use zero_cost_logger::*;
use rdev::Key;

use crate::platform::Platform;

use crate::common::structs::KeyboardModifiers;
use crate::settings::keyboard_layouts::LayoutItem;

use super::KeyInsert;

pub struct KeyboardLayouts {
  platform: &'static Platform,

  pub items: HashMap<String, LayoutItem>,
}

impl KeyboardLayouts {
  pub fn new(platform: &'static Platform) -> Self {
    KeyboardLayouts {
      platform,
      items: HashMap::new(),
    }
  }

  pub fn init(&mut self) {
    log!("<$>Keyboard Layouts Settings</>: Init");
  }

  pub fn get_keyboard_layouts(&self) -> &HashMap<String, LayoutItem> {
    &self.items
  }

  pub fn add(&mut self, name: &str) {
    let layout = LayoutItem::new(name);

    self.items.insert(name.to_owned(), layout);
  }

  // Yes, we need here tuple not just Some/None, the second return means the char existing in some other layout
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

  pub fn check_keyboard_setup(&self) -> anyhow::Result<()> {
    let layouts = &self.platform.keyboard_layouts.borrow().items;

    for layout in layouts {
      let setup = self.items.get(&layout.name);

      if setup.is_none() {
        error!("<$>Keyboard Layouts Settings</>: Keyboard layout now found: {}", layout.name);
        anyhow::bail!("Keyboard layout now found: {}", layout.name);
      }
    }

    Ok(())
  }
}
