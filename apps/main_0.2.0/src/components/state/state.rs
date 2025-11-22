use crate::components::state::{ApplicationState, KeyboardState};

use crate::platform::Platform;
use crate::settings::Settings;

pub struct State {
  platform: &'static Platform,
  settings: &'static Settings,

  pub application: ApplicationState,
  pub keyboard: KeyboardState,
}

impl State {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    State {
      platform,
      settings,

      application: ApplicationState::new(),
      keyboard: KeyboardState::new(platform, settings),
    }
  }
}
