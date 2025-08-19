use std::sync::Arc;

use rust_logger::*;

use crate::platform::Platform;

use crate::components::{
  event_hub::EventHub,
  controllers::UserInputController,
  detectors::{CommandDetector, HotkeyDetector},
  executor::Executor,
};

use crate::settings::Layouts;

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

    let platform = Platform::get_instance();

    UserInputController::new(hub.clone()).start();

    CommandDetector::new(hub.clone()).start();
    HotkeyDetector::new(hub.clone()).start();

    Executor::new(hub.clone()).start();

    let mut layouts = Layouts::new();

    for layout in &platform.keyboard_layouts.items {
      layouts.add(&layout.name);
    }

    log!("<purple>Application</>: Ready");

    Ok(())
  }
}
