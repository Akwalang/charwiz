use std::fs;
use std::path::Path;
use std::collections::HashMap;

use rust_logger::*;
use rdev::Key;

use crate::settings::keyboard_layouts::KeyItem;

use crate::common::structs::KeyboardEventSnapshot;

use crate::constants::LAYOUTS_FOLDER;

pub struct LayoutItem {
  pub name: String,
  pub chars: HashMap<char, KeyboardEventSnapshot>,
  pub keys: HashMap<Key, KeyItem>,
}

impl LayoutItem {
  pub fn new(name: &str) -> Self {
    let name = name.to_owned();

    let items = Self::load(&name).unwrap_or(Vec::new());

    let chars = Self::raw_to_chars_map(&items);
    let keys = Self::raw_to_keys_map(items);

    LayoutItem { name, chars, keys }
  }

  fn load(name: &str) -> anyhow::Result<Vec<KeyItem>> {
    let content = Self::read_file(name)?;
    
    Self::parse_json(name, content)
  }

  fn read_file(name: &str) -> anyhow::Result<String> {
    let src = format!("{}/{}.json", LAYOUTS_FOLDER, name);

    log!("<$>Settings::KeyboardLayouts</>: Loading keyboard layout for \"<&>{}</>\": <i&>{}</>", name, src);

    let path = Path::new(&src);

    let content = fs::read_to_string(&path);

    if let Err(e) = content {
      error!("<$>Settings::KeyboardLayouts</>: Failed to load keyboard layout for \"<&>{}</>\": <i&>{}</>", name, src);
      return Err(anyhow::anyhow!(e.to_string()));
    }
    
    Ok(content.unwrap())
  }

  fn parse_json(name: &str, content: String) -> anyhow::Result<Vec<KeyItem>> {
    let items = serde_json::from_str::<Vec<KeyItem>>(&content);

    if let Err(e) = items {
      error!("<$>Settings::KeyboardLayouts</>: Invalid keyboard layout JSON for \"<&>{}</>\": <i&>{}</> -> <->{}</>", name, content, e);
      return Err(anyhow::anyhow!(e.to_string()));
    }

    Ok(items.unwrap())
  }

  fn raw_to_chars_map(items: &Vec<KeyItem>) -> HashMap<char, KeyboardEventSnapshot> {
    let size: usize = items.iter().map(|i| i.insert.len()).sum();

    let mut map: HashMap<char, KeyboardEventSnapshot> = HashMap::with_capacity(size);

    for item in items {
      for ins in &item.insert {
        let value = KeyboardEventSnapshot::new(Some(item.key), ins.modifiers);

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
