use rdev::Key;

#[derive(Debug)]
pub enum KeyEvent {
  KeyDown(Key),
  KeyUp(Key),
}
