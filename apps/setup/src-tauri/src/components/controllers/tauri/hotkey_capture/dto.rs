use serde::Serialize;
use settings_serde::structs::KeyboardSnapshotRaw;

pub enum HotkeyDto {
  UnsubscribeListener(HotkeyUnsubscribeListener),
  SendSnapshot(HotkeySendSnapshot),
}

#[derive(Serialize, Clone)]
pub struct HotkeyUnsubscribeListener {
  pub listener_id: String,
}

#[derive(Serialize, Clone)]
pub struct HotkeySendSnapshot {
  pub listener_id: String,
  pub snapshot: KeyboardSnapshotRaw,
}
