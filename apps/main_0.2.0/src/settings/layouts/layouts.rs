use std::collections::HashMap;

use crate::settings::Layout;

pub struct Layouts {
  items: HashMap<String, Layout>,
}

impl Layouts {
  pub fn new() -> Self {
    Layouts {
      items: HashMap::new(),
    }
  }

  pub fn add(&mut self, name: &str) {
    let layout = Layout::new(name);

    self.items.insert(name.to_owned(), layout);
  }

  pub fn get_inserted_char() -> char {
    'q'
  }
}
