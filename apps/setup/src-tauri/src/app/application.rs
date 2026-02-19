use std::sync::{Arc, Mutex};

use crate::components::state::State;
use crate::components::controllers::{tauri as commands};

pub struct Application {
  state: Arc<Mutex<State>>,
}

impl Application {
  pub fn new() -> Self {
    Application {
      state: Arc::new(Mutex::new(State::new())),
    }
  }

  pub async fn run(&self) -> anyhow::Result<()> {
    tauri::Builder::default()
      .manage(self.state.clone())
      .plugin(tauri_plugin_opener::init())
      .invoke_handler(tauri::generate_handler![
        commands::start_hotkey_capture,
        commands::stop_hotkey_capture,
      ])
      .run(tauri::generate_context!())?;
      
    Ok(())
  }
}
