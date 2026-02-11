use std::collections::HashMap;

#[cfg(feature = "logger")]
use logger::*;

use rdev::Key;

use settings_core::structs::KeyboardSnapshot;
use settings_core::settings::keyboard_layouts::KeyItem;

use settings_serde::utils::load_keyboard_layout;

use crate::constants::LAYOUTS_FOLDER;

pub struct LayoutItem {
  pub chars: HashMap<char, KeyboardSnapshot>,
  pub keys: HashMap<Key, KeyItem>,
}

impl LayoutItem {
  pub fn new(name: &str) -> Self {
    let items = Self::load(&name).unwrap_or_else(|_| panic!("Can't load layout settings"));

    let chars = Self::raw_to_chars_map(&items);
    let keys = Self::raw_to_keys_map(items);

    LayoutItem { chars, keys }
  }

  fn load(name: &str) -> anyhow::Result<Vec<KeyItem>> {
    #[cfg(feature = "logger")]
    log!("<$>Keyboard Layouts Settings</>: Loading layout: <i&>{}</>", name);

    let src = format!("{}/{}.json", LAYOUTS_FOLDER, name);

    let result = load_keyboard_layout(&src);

    #[cfg(feature = "logger")]
    if let Err(ref e) = result {
      error!("<$>Keyboard Layouts Settings</>: Failed to load layout settings. Error: <i->{}</>", e.to_string());
    };

    result
  }

  fn raw_to_chars_map(items: &Vec<KeyItem>) -> HashMap<char, KeyboardSnapshot> {
    let size: usize = items.iter().map(|i| i.insert.len()).sum();

    let mut map: HashMap<char, KeyboardSnapshot> = HashMap::with_capacity(size);

    for item in items {
      for ins in &item.insert {
        // prefer regular keys over numpad keys
        if map.contains_key(&ins.char) { continue; }

        let value = KeyboardSnapshot::new(Some(item.key), ins.modifiers);

        map.insert(ins.char, value);
      }
    }

    map
  }

  fn raw_to_keys_map(items: Vec<KeyItem>) -> HashMap<Key, KeyItem> {
    let mut map: HashMap<Key, KeyItem> = HashMap::with_capacity(items.len());

    for item in items {
      map.insert(item.key, item);
    }

    map
  }
}
