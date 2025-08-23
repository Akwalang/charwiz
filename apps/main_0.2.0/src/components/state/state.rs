use crate::components::state::{ApplicationState, KeyboardState};

pub struct State {
  pub application: ApplicationState,
  pub keyboard: KeyboardState,
}

impl State {
  pub fn new() -> Self {
    State {
      application: ApplicationState::new(),
      keyboard: KeyboardState::new(),
    }
  }
}
