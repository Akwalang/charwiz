use std::sync::{Arc, Mutex};

use rust_logger::*;

use crate::platform::Platform;
use crate::settings::Settings;

use crate::components::state::{State, ApplicationStatus};
use crate::components::event_hub::EventHub;

use crate::components::controllers::UserInputController;
use crate::components::detectors::{AutoConvertDetector, CommandDetector, HotkeyDetector};

use crate::components::executor::Executor;

pub struct Application {
  platform: &'static Platform,
  settings: &'static Settings,

  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,
}

impl Application {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    let state = State::new(platform, settings);
    let event_hub = EventHub::new();

    let state = Arc::new(Mutex::new(state));
    let event_hub = Arc::new(event_hub);

    Application { platform, settings, state, event_hub }
  }

  pub async fn run(&self) {
    log!("<$>Application</>: Starting...");

    let state = &self.state;
    let event_hub = &self.event_hub;

    UserInputController::new(state, event_hub).init();

    AutoConvertDetector::new(self.settings, state.clone(), event_hub.clone()).init();
    CommandDetector::new(self.settings, state.clone(), event_hub.clone()).init();
    HotkeyDetector::new(self.settings, state.clone(), event_hub.clone()).init();

    let executor = Executor::new(
      self.platform,
      self.settings,
      state.clone(),
      event_hub.clone(),
    );

    executor.init();

    state.lock().unwrap().application.set_status(ApplicationStatus::Active);

    log!("<$>Application</>: Ready");

    loop {
      tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
  }
}
