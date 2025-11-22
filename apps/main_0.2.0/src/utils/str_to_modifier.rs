use rdev::Key;

pub fn str_to_modifier(s: &str) -> Option<Key> {
  match s.to_lowercase().as_str() {
    "alt" => Some(Key::Alt),
    "altgr" => Some(Key::AltGr),
    "controlleft" => Some(Key::ControlLeft),
    "controlright" => Some(Key::ControlRight),
    "shiftleft" => Some(Key::ShiftLeft),
    "shiftright" => Some(Key::ShiftRight),
    "metaleft" => Some(Key::MetaLeft),
    "metaright" => Some(Key::MetaRight),
    _ => None,
  }
}
