use std::collections::HashMap;

#[derive(Debug)]
pub struct SettingsKeyboardLayout {
  pub into_num: HashMap<char, u8>,
  pub into_char: HashMap<u8, char>,
}

impl Into<SettingsKeyboardLayout> for String {
  fn into(self) -> SettingsKeyboardLayout {
    let mut into_num: HashMap<char, u8> = HashMap::new();
    let mut into_char: HashMap<u8, char> = HashMap::new();

    for (i, c) in self.chars().enumerate() {
      into_num.insert(c, i as u8);
      into_char.insert(i as u8, c);
    }

    SettingsKeyboardLayout { into_num, into_char }
  }
}
