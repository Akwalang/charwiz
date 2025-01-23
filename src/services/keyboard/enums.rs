use rdev::Key;

pub enum KeyEvent {
  KeyDown(Key),
  KeyUp(Key),
}
