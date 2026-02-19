use std::sync::{Arc, Mutex};
use tauri::{Emitter, State};

use logger::*;

use settings_serde::structs::{KeyboardSnapshotRaw, KeyboardModifiersRaw};

use crate::components::state::{State as AppState};
use crate::components::state::structs::Stream;

use super::dto::{HotkeyUnsubscribeListenerDto, HotkeySendSnapshotDto};

#[tauri::command]
pub async fn start_hotkey_capture(
  app: tauri::AppHandle,
  state: State<'_, Arc<Mutex<AppState>>>,
  listener_id: String,
) -> Result<(), String> {
  log!("<$>Tauri Controller</>: Income event <!>start_hotkey_capture</>. Listener id: <&>{}</>", listener_id);

  let mut lock = state.lock().unwrap();

  'unsubscribe: {
    let Some(stream) = lock.hotkey_stream.as_ref() else {
      break 'unsubscribe;
    };

    log!("<$>Tauri Controller</>: Unsubscribe from <!>stream_hotkey_capture</>. Listener id: <&>{}</>", stream.listener_id);

    stream.handle.abort();

    let payload = HotkeyUnsubscribeListenerDto::new(stream.listener_id.clone());

    let _ = app.emit("stream_hotkey_capture", payload);

    lock.hotkey_stream = None;
  };

  #[allow(unused_labels)]
  'subscribe: {
    log!("<$>Tauri Controller</>: Subscribe to <!>stream_hotkey_capture</>. Listener id: <&>{}</>", listener_id);

    let id: String = listener_id.clone();

    let handle = tokio::spawn(async move {
      loop {
        let payload = HotkeySendSnapshotDto::new(
          id.clone(),
          KeyboardSnapshotRaw {
            key: Some(rdev::Key::KeyM),
            modifiers: KeyboardModifiersRaw::default(),
          },
        );

        let _ = app.emit("stream_hotkey_capture", payload);

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
      }
    });

    lock.hotkey_stream = Some(Stream { listener_id, handle });
  };

  Ok(())
}

#[tauri::command]
pub fn stop_hotkey_capture(
  state: State<'_, Arc<Mutex<AppState>>>,
  listener_id: String,
) {
  let lock = state.lock().unwrap();

  log!("<$>Tauri Controller</>: Income event <!>stop_hotkey_capture</>. Listener id: <&>{}</>", listener_id);

  let Some(stream) = lock.hotkey_stream.as_ref() else {
    return;
  };

  if stream.listener_id == listener_id {
    log!("<$>Tauri Controller</>: Unsubscribe from <!>stream_hotkey_capture</>. Listener id: <&>{}</>", stream.listener_id);

    stream.handle.abort();
  }
}
