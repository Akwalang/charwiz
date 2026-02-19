use std::sync::{Arc, Mutex};
use tauri::{Emitter, State};

use logger::*;

use settings_serde::structs::{KeyboardSnapshotRaw, KeyboardModifiersRaw};

use crate::components::state::{State as AppState};
use super::dto;

#[tauri::command]
pub async fn start_hotkey_capture(
  app: tauri::AppHandle,
  state: State<'_, Arc<Mutex<AppState>>>,
  listener_id: String,
) -> Result<(), String> {
  log!("<$>Tauri Controller</>: Income event <!>start_hotkey_capture</>. Listener id: <&>{}</>", listener_id);

  {
    let mut lock = state.lock().unwrap();

    if let Some(current_id) = lock.hotkey_listener_id.as_ref() {
      if *current_id != listener_id {
        let payload = dto::HotkeyUnsubscribeListener {
          listener_id: current_id.clone(),
        };

        let _ = app.emit("stream_hotkey_capture", payload);
      }
    }

    lock.hotkey_listener_id = Some(listener_id.clone());
  }

  tokio::spawn(async move {
    loop {
      let payload = dto::HotkeySendSnapshot {
        listener_id: listener_id.clone(),
        snapshot: KeyboardSnapshotRaw {
          key: Some(rdev::Key::KeyM),
          modifiers: KeyboardModifiersRaw::default(),
        },
      };

      let _ = app.emit("stream_hotkey_capture", payload);

      tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
  });

  Ok(())
}

#[tauri::command]
pub fn stop_hotkey_capture(
  state: State<'_, Arc<Mutex<AppState>>>,
  listener_id: String,
) -> String {
  let lock = state.lock().unwrap();

  log!("<$>Tauri Controller</>: Income event <!>stop_hotkey_capture</>. Listener id: <&>{}</>", listener_id);
  format!("{} :: stop_hotkey_capture!!!", lock.name)
}
