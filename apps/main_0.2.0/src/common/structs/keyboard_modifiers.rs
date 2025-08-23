use rdev::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyboardModifiers(u8);

impl KeyboardModifiers {
  pub const CONTROL_LEFT: u8 = 1 << 0;
  pub const CONTROL_RIGHT: u8 = 1 << 1;
  pub const SHIFT_LEFT: u8 = 1 << 2;
  pub const SHIFT_RIGHT: u8 = 1 << 3;
  pub const ALT: u8 = 1 << 4;
  pub const ALT_GR: u8 = 1 << 5;

  pub const CONTROL_ANY: u8 = Self::CONTROL_LEFT | Self::CONTROL_RIGHT;
  pub const SHIFT_ANY: u8 = Self::SHIFT_LEFT | Self::SHIFT_RIGHT;

  #[inline(always)]
  pub fn new() -> Self {
    KeyboardModifiers(0)
  }

  pub fn add_key(&mut self, key: &Key) {
    match key {
      Key::ControlLeft => self.0 |= Self::CONTROL_LEFT,
      Key::ControlRight => self.0 |= Self::CONTROL_RIGHT,
      Key::ShiftLeft => self.0 |= Self::SHIFT_LEFT,
      Key::ShiftRight => self.0 |= Self::SHIFT_RIGHT,
      Key::Alt => self.0 |= Self::ALT,
      Key::AltGr => self.0 |= Self::ALT_GR,
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
      _ => false,
    }
  }
}
