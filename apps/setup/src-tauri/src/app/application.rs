use std::sync::{Arc, Mutex};

use crate::components::state::State;
use crate::components::event_hub::EventHub;

use crate::components::controllers::{tauri as commands};

pub struct Application {
  state: Arc<Mutex<State>>,
  event_hub: Arc<Mutex<EventHub>>,
}

impl Application {
  pub fn new() -> Self {
    let state = State::new();
    let event_hub = EventHub::new();

    Application {
      state: Arc::new(Mutex::new(state)),
      event_hub: Arc::new(Mutex::new(event_hub)),
    }
  }

  pub async fn run(&self) {
    self.start_tauri_app();
  }

  fn start_tauri_app(&self) {
    let _ = tauri::Builder::default()
      .manage(self.state.clone())
      .plugin(tauri_plugin_opener::init())
      .invoke_handler(tauri::generate_handler![
        commands::start_hotkey_capture,
        commands::stop_hotkey_capture,
      ])
      .run(tauri::generate_context!());
  }
}
