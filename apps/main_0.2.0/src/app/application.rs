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
  status: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,
}

impl Application {
  pub fn new() -> Self {
    let status = Arc::new(Mutex::new(State::new()));
    let event_hub = Arc::new(EventHub::new());

    Application { status, event_hub }
  }

  pub fn run(&self) -> anyhow::Result<()> {
    log!("<$>Application</>: Starting...");

    let status = &self.status;
    let event_hub = &self.event_hub;

    Platform::init();
    Settings::init();

    UserInputController::new(status, event_hub).init();

    CommandDetector::new(status, event_hub).init();
    HotkeyDetector::new(status, event_hub).init();

    Executor::new(status, event_hub).init();

    log!("<$>Application</>: Ready");

    Ok(())
  }
}
