use std::sync::{Arc, Mutex};

use rust_logger::*;

use crate::platform::Platform;
use crate::settings::Settings;

use crate::components::{
  event_hub::EventHub,
  controllers::UserInputController,
  detectors::{CommandDetector, HotkeyDetector},
  executor::Executor,
  state::State,
};

pub struct Application {
  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,
}

impl Application {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    let state = State::new(platform, settings);
    let event_hub = EventHub::new();

    let state = Arc::new(Mutex::new(state));
    let event_hub = Arc::new(event_hub);

    Application { state, event_hub }
  }

  pub fn run(&self) -> anyhow::Result<()> {
    log!("<$>Application</>: Starting...");

    let state = &self.state;
    let event_hub = &self.event_hub;

    UserInputController::new(state, event_hub).init();

    CommandDetector::new(state, event_hub).init();
    HotkeyDetector::new(state, event_hub).init();

    Executor::new(state, event_hub).init();

    log!("<$>Application</>: Ready");

    Ok(())
  }
}
