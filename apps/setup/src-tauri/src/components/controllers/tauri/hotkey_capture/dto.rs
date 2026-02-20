use serde::Serialize;
use settings_serde::structs::KeyboardSnapshotRaw;

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
enum HotkeyEventType {
  Unsubscribe,
  Snapshot,
}  

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HotkeyUnsubscribeListenerDto {
  event_type: HotkeyEventType,
  listener_id: String,
}

impl HotkeyUnsubscribeListenerDto {
  pub fn new(listener_id: String) -> Self {
    let event_type = HotkeyEventType::Unsubscribe;

    Self { event_type, listener_id }
  }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HotkeySendSnapshotDto {
  event_type: HotkeyEventType,
  listener_id: String,
  snapshot: KeyboardSnapshotRaw,
}

impl HotkeySendSnapshotDto {
  pub fn new(listener_id: String, snapshot: KeyboardSnapshotRaw) -> Self {
    let event_type = HotkeyEventType::Snapshot;

    Self { event_type, listener_id, snapshot }
  }
}
