use std::fs;
use std::path::Path;

use rust_logger::*;

use crate::settings::layouts::KeyItem;

use crate::constants::LAYOUTS_FOLDER;

pub struct Layout {
  name: String,
  keys: Vec<KeyItem>,
}

impl Layout {
  pub fn new(name: &str) -> Self {
    Layout {
      name: name.to_string(),
      keys: Self::load(name).unwrap_or(vec![]),
    }
  }

  fn load(name: &str) -> anyhow::Result<Vec<KeyItem>> {
    let src = format!("{}/{}.json", LAYOUTS_FOLDER, name);

    log!("<purple>Settings</>: Loading keyboard layout for \"<cyan>{}</>\": <cyan,i>{}</>", name, src);

    let path = Path::new(&src);

    let content = fs::read_to_string(&path);
    
    let content = match content {
      Ok(c) => c,
      Err(e) => {
        error!("<purple>Settings</>: Failed to load keyboard layout for \"<cyan>{}</>\": <cyan,i>{}</>", name, src);
        return Err(anyhow::anyhow!(e.to_string()))
      }
    };

    let items: Vec<KeyItem> = match serde_json::from_str(&content) {
      Ok(data) => data,
      Err(e) => {
        error!("<purple>Settings</>: Invalid keyboard layout JSON for \"<cyan>{}</>\": <cyan,i>{}</> -> <red>{}</>", name, src, e);
        return Err(anyhow::anyhow!(e.to_string()))
      }
    };

    Ok(items)
  }
}
