use std::sync::Arc;

use rust_logger::*;

use crate::platform::Platform;

use crate::components::{
  event_hub::EventHub,
  controllers::UserInputController,
  detectors::{CommandDetector, HotkeyDetector},
  executor::Executor,
};

use crate::settings::Settings;

pub struct Application {
  event_hub: Arc<EventHub>,
}

impl Application {
  pub fn new() -> Self {
    let event_hub = Arc::new(EventHub::new());

    Application { event_hub }
  }

  pub fn run(&self) -> anyhow::Result<()> {
    log!("<purple>Application</>: Starting...");

    let hub = &self.event_hub;

    Platform::init();
    Settings::init();

    UserInputController::new(hub).init();

    CommandDetector::new(hub).init();
    HotkeyDetector::new(hub).init();

    Executor::new(hub).init();

    log!("<purple>Application</>: Ready");

    Ok(())
  }
}
