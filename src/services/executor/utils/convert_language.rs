use std::collections::HashMap;
use std::sync::OnceLock;

static MAPPING: OnceLock<HashMap<char, char>> = OnceLock::new();

fn init_mapping() -> HashMap<char, char> {
  let ru = "йцукенгшщзхъфывапролджэячсмитьбю.ёЙЦУКЕНГШЩЗХЪФЫВАПРОЛДЖЭЯЧСМИТЬБЮ,Ё!@#$%^&*()";
  let en = "qwertyuiop[]asdfghjkl;'zxcvbnm,./`QWERTYUIOP{}ASDFGHJKL:\"ZXCVBNM<>?~!@#$%^&*()";

  let mut mapping = HashMap::new();

  for (i, c) in ru.chars().enumerate() {
    mapping.insert(c, en.chars().nth(i).unwrap());
  }

  for (i, c) in en.chars().enumerate() {
    mapping.insert(c, ru.chars().nth(i).unwrap());
  }

  mapping
}

pub fn convert_language(value: String) -> String {
  let mapping = MAPPING.get_or_init(init_mapping);

  let mut result = String::with_capacity(value.len());

  for a in value.chars() {
    let b = match mapping.get(&a) {
      Some(b) => *b,
      None => a,
    };

    result.push(b);
  }

  result
}
