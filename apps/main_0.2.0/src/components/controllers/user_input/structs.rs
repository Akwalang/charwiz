use rdev::Key;

pub struct StickyKeys(u16);

impl StickyKeys {
  const META_LEFT: u16 = 1 << 0;
  const META_RIGHT: u16 = 1 << 1;
  const CONTROL_LEFT: u16 = 1 << 2;
  const CONTROL_RIGHT: u16 = 1 << 3;
  const SHIFT_LEFT: u16 = 1 << 4;
  const SHIFT_RIGHT: u16 = 1 << 5;
  const ALT: u16 = 1 << 6;
  const ALT_GR: u16 = 1 << 7;
  const UP_ARROW: u16 = 1 << 8;
  const DOWN_ARROW: u16 = 1 << 9;
  const LEFT_ARROW: u16 = 1 << 10;
  const RIGHT_ARROW: u16 = 1 << 11;
  const PAGE_UP: u16 = 1 << 12;
  const PAGE_DOWN: u16 = 1 << 13;
  const HOME: u16 = 1 << 14;
  const END: u16 = 1 << 15;

  #[inline]
  pub fn new() -> Self {
    StickyKeys(0)
  }

  pub fn add_key(&mut self, key: &Key) {
    match key {
      Key::MetaLeft => self.0 |= Self::META_LEFT,
      Key::MetaRight => self.0 |= Self::META_RIGHT,
      Key::ControlLeft => self.0 |= Self::CONTROL_LEFT,
      Key::ControlRight => self.0 |= Self::CONTROL_RIGHT,
      Key::ShiftLeft => self.0 |= Self::SHIFT_LEFT,
      Key::ShiftRight => self.0 |= Self::SHIFT_RIGHT,
      Key::Alt => self.0 |= Self::ALT,
      Key::AltGr => self.0 |= Self::ALT_GR,
      Key::UpArrow => self.0 |= Self::UP_ARROW,
      Key::DownArrow => self.0 |= Self::DOWN_ARROW,
      Key::LeftArrow => self.0 |= Self::LEFT_ARROW,
      Key::RightArrow => self.0 |= Self::RIGHT_ARROW,
      Key::PageUp => self.0 |= Self::PAGE_UP,
      Key::PageDown => self.0 |= Self::PAGE_DOWN,
      Key::Home => self.0 |= Self::HOME,
      Key::End => self.0 |= Self::END,
      _ => {},
    }
  }

  pub fn remove_key(&mut self, key: &Key) {
    match key {
      Key::MetaLeft => self.0 &= !Self::META_LEFT,
      Key::MetaRight => self.0 &= !Self::META_RIGHT,
      Key::ControlLeft => self.0 &= !Self::CONTROL_LEFT,
      Key::ControlRight => self.0 &= !Self::CONTROL_RIGHT,
      Key::ShiftLeft => self.0 &= !Self::SHIFT_LEFT,
      Key::ShiftRight => self.0 &= !Self::SHIFT_RIGHT,
      Key::Alt => self.0 &= !Self::ALT,
      Key::AltGr => self.0 &= !Self::ALT_GR,
      Key::UpArrow => self.0 &= !Self::UP_ARROW,
      Key::DownArrow => self.0 &= !Self::DOWN_ARROW,
      Key::LeftArrow => self.0 &= !Self::LEFT_ARROW,
      Key::RightArrow => self.0 &= !Self::RIGHT_ARROW,
      Key::PageUp => self.0 &= !Self::PAGE_UP,
      Key::PageDown => self.0 &= !Self::PAGE_DOWN,
      Key::Home => self.0 &= !Self::HOME,
      Key::End => self.0 &= !Self::END,
      _ => {},
    }
  }

  pub fn is_pressed(&self, key: &Key) -> bool {
    match key {
      Key::MetaLeft => (self.0 & Self::META_LEFT) != 0,
      Key::MetaRight => (self.0 & Self::META_RIGHT) != 0,
      Key::ControlLeft => (self.0 & Self::CONTROL_LEFT) != 0,
      Key::ControlRight => (self.0 & Self::CONTROL_RIGHT) != 0,
      Key::ShiftLeft => (self.0 & Self::SHIFT_LEFT) != 0,
      Key::ShiftRight => (self.0 & Self::SHIFT_RIGHT) != 0,
      Key::Alt => (self.0 & Self::ALT) != 0,
      Key::AltGr => (self.0 & Self::ALT_GR) != 0,
      Key::UpArrow => (self.0 & Self::UP_ARROW) != 0,
      Key::DownArrow => (self.0 & Self::DOWN_ARROW) != 0,
      Key::LeftArrow => (self.0 & Self::LEFT_ARROW) != 0,
      Key::RightArrow => (self.0 & Self::RIGHT_ARROW) != 0,
      Key::PageUp => (self.0 & Self::PAGE_UP) != 0,
      Key::PageDown => (self.0 & Self::PAGE_DOWN) != 0,
      Key::Home => (self.0 & Self::HOME) != 0,
      Key::End => (self.0 & Self::END) != 0,
      _ => false,
    }
  }

  pub fn is_sticky_key(key: &Key) -> bool {
    match key {
      Key::MetaLeft | Key::MetaRight => true,
      Key::ControlLeft | Key::ControlRight => true,
      Key::ShiftLeft | Key::ShiftRight => true,
      Key::Alt | Key::AltGr => true,
      Key::UpArrow | Key::DownArrow => true,
      Key::LeftArrow | Key::RightArrow => true,
      Key::PageUp | Key::PageDown => true,
      Key::Home | Key::End => true,
      _ => false,
    }
  }
}
