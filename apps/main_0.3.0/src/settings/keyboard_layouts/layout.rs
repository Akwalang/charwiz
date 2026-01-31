use std::collections::HashMap;

use rust_logger::*;
use rdev::Key;

use super::{KeyItemRaw, KeyItem};

use crate::common::structs::KeyboardSnapshot;

use crate::constants::LAYOUTS_FOLDER;

use crate::utils;

pub struct LayoutItem {
  pub name: String,
  pub chars: HashMap<char, KeyboardSnapshot>,
  pub keys: HashMap<Key, KeyItem>,
}

impl LayoutItem {
  pub fn new(name: &str) -> Self {
    let name = name.to_owned();

    let items = Self::load(&name).unwrap_or_else(|_| panic!("Can't load layout settings"));

    let chars = Self::raw_to_chars_map(&items);
    let keys = Self::raw_to_keys_map(items);

    LayoutItem { name, chars, keys }
  }

  fn load(name: &str) -> anyhow::Result<Vec<KeyItem>> {
    log!("<$>Keyboard Layouts Settings</>: Loading layout: <i&>{}</>", name);

    let src = format!("{}/{}.json", LAYOUTS_FOLDER, name);

    let raw = utils::read_json::<Vec<KeyItemRaw>>(&src);

    if let Err(e) = raw {
      error!("<$>Keyboard Layouts Settings</>: Failed to load layout settings. Error: <i->{}</>", e.to_string());
      return Err(e);
    };

    let result = raw.unwrap_or(vec![]).into_iter().map(Into::into).collect();

    Ok(result)
  }

  fn raw_to_chars_map(items: &Vec<KeyItem>) -> HashMap<char, KeyboardSnapshot> {
    let size: usize = items.iter().map(|i| i.insert.len()).sum();

    let mut map: HashMap<char, KeyboardSnapshot> = HashMap::with_capacity(size);

    for item in items {
      for ins in &item.insert {
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
