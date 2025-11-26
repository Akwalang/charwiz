use rdev::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyboardModifiers(pub u8);

impl KeyboardModifiers {
  pub const NONE: u8 = 0;

  pub const CONTROL_LEFT: u8 = 1 << 0;
  pub const CONTROL_RIGHT: u8 = 1 << 1;
  pub const SHIFT_LEFT: u8 = 1 << 2;
  pub const SHIFT_RIGHT: u8 = 1 << 3;
  pub const ALT: u8 = 1 << 4;
  pub const ALT_GR: u8 = 1 << 5;
  pub const META_LEFT: u8 = 1 << 6;
  pub const META_RIGHT: u8 = 1 << 7;

  pub const CONTROL_ANY: u8 = Self::CONTROL_LEFT | Self::CONTROL_RIGHT;
  pub const SHIFT_ANY: u8 = Self::SHIFT_LEFT | Self::SHIFT_RIGHT;

  #[inline(always)]
  pub fn new() -> Self {
    KeyboardModifiers(Self::NONE)
  }

  pub fn add_key(&mut self, key: &Key) {
    match key {
      Key::ControlLeft => self.0 |= Self::CONTROL_LEFT,
      Key::ControlRight => self.0 |= Self::CONTROL_RIGHT,
      Key::ShiftLeft => self.0 |= Self::SHIFT_LEFT,
      Key::ShiftRight => self.0 |= Self::SHIFT_RIGHT,
      Key::Alt => self.0 |= Self::ALT,
      Key::AltGr => self.0 |= Self::ALT_GR,
      Key::MetaLeft => self.0 |= Self::META_LEFT,
      Key::MetaRight => self.0 |= Self::META_RIGHT,
      _ => {},
    }
  }

  pub fn remove_key(&mut self, key: &Key) {
    match key {
      Key::ControlLeft => self.0 &= !Self::CONTROL_LEFT,
      Key::ControlRight => self.0 &= !Self::CONTROL_RIGHT,
      Key::ShiftLeft => self.0 &= !Self::SHIFT_LEFT,
      Key::ShiftRight => self.0 &= !Self::SHIFT_RIGHT,
      Key::Alt => self.0 &= !Self::ALT,
      Key::AltGr => self.0 &= !Self::ALT_GR,
      Key::MetaLeft => self.0 &= !Self::META_LEFT,
      Key::MetaRight => self.0 &= !Self::META_RIGHT,
      _ => {},
    }
  }

  pub fn is_pressed(&self, key: &Key) -> bool {
    match key {
      Key::ControlLeft => (self.0 & Self::CONTROL_LEFT) != 0,
      Key::ControlRight => (self.0 & Self::CONTROL_RIGHT) != 0,
      Key::ShiftLeft => (self.0 & Self::SHIFT_LEFT) != 0,
      Key::ShiftRight => (self.0 & Self::SHIFT_RIGHT) != 0,
      Key::Alt => (self.0 & Self::ALT) != 0,
      Key::AltGr => (self.0 & Self::ALT_GR) != 0,
      Key::MetaLeft => (self.0 & Self::META_LEFT) != 0,
      Key::MetaRight => (self.0 & Self::META_RIGHT) != 0,
      _ => false,
    }
  }

  #[inline(always)]
  pub fn is_any_pressed(&self, modifiers: u8) -> bool {
    (self.0 & modifiers) != 0
  }

  pub fn is_modifier(key: &Key) -> bool {
    match key {
      Key::ControlLeft | Key::ControlRight => true,
      Key::ShiftLeft | Key::ShiftRight => true,
      Key::Alt | Key::AltGr => true,
      Key::MetaLeft | Key::MetaRight => true,
      _ => false,
    }
  }

  pub fn compare_to_release(a: &KeyboardModifiers, b: &KeyboardModifiers) -> KeyboardModifiers {
    // a     01001001
    // b     00101100
    // xor   01100101
    // and   01000001

    KeyboardModifiers((a.0 ^ b.0) & a.0)
  }

  pub fn compare_to_press(a: &KeyboardModifiers, b: &KeyboardModifiers) -> KeyboardModifiers {
    // a     01001001
    // b     00101100
    // xor   01100101
    // and   00100100

    KeyboardModifiers((a.0 ^ b.0) & b.0)
  }
}

pub struct KeyboardModifiersIter {
  modifiers: KeyboardModifiers,
  index: u8,
}

impl KeyboardModifiersIter {
  pub fn new(modifiers: KeyboardModifiers) -> Self {
    Self { modifiers, index: 0 }
  }
}

impl Iterator for KeyboardModifiersIter {
  type Item = Key;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      if self.index >= u8::BITS as u8 { return None };

      let key: u8 = 1 << self.index;

      self.index += 1;

      if self.modifiers.0 & key == 0 { continue; }

      return match key {
        KeyboardModifiers::CONTROL_LEFT => Some(Key::ControlLeft),
        KeyboardModifiers::CONTROL_RIGHT => Some(Key::ControlRight),
        KeyboardModifiers::SHIFT_LEFT => Some(Key::ShiftLeft),
        KeyboardModifiers::SHIFT_RIGHT => Some(Key::ShiftRight),
        KeyboardModifiers::ALT => Some(Key::Alt),
        KeyboardModifiers::ALT_GR => Some(Key::AltGr),
        KeyboardModifiers::META_LEFT => Some(Key::MetaLeft),
        KeyboardModifiers::META_RIGHT => Some(Key::MetaRight),
        _ => None,
      }
    }
  }
}

// Реализуем IntoIterator для удобства
impl IntoIterator for KeyboardModifiers {
  type Item = Key;
  type IntoIter = KeyboardModifiersIter;
  
  fn into_iter(self) -> Self::IntoIter {
    KeyboardModifiersIter::new(self)
  }
}
