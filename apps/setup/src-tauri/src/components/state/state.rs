use super::structs::Stream;

pub struct State {
  pub hotkey_stream: Option<Stream>,
}

impl State {
  pub fn new() -> Self {
    State {
      hotkey_stream: None,
    }
  }
}
