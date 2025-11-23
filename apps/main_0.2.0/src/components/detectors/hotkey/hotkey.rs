use std::sync::{Arc, Mutex};
use std::collections::HashSet;

use rust_logger::*;
use rdev::{EventType, Key};

use crate::components::event_hub::{EventHub, InputEvent, CommandEvent};
use crate::components::state::State;

use crate::settings::Settings;

use crate::common::structs::KeyboardEventSnapshot;

pub struct HotkeyDetector {
  settings: &'static Settings,
  
  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,
  captured_keys: Mutex<KeyboardEventSnapshot>,
}

impl HotkeyDetector {
  pub fn new(settings: &'static Settings, state: Arc<Mutex<State>>, event_hub: Arc<EventHub>) -> Arc<Self> {
    Arc::new(HotkeyDetector {
      settings,
      state: state,
      event_hub: event_hub,
      captured_keys: Mutex::new(KeyboardEventSnapshot::default()),
    })
  }

  pub fn init(self: &Arc<Self>) {
    log!("<$>HotKeyDetector</>: Init");

    self.subscribe();
  }

  fn subscribe(self: &Arc<Self>) {
    let mut input_rx = self.event_hub.input_stream();

    let this = Arc::clone(self);

    async_std::task::spawn(async move {
      while let Ok(event) = input_rx.recv().await {
        this.process_event(event);
      }
    });
  }

  fn process_event(&self, event: InputEvent) {
    if !Self::is_trackable_event(&event) { return; }
    
    let state: std::sync::MutexGuard<'_, State> = self.state.lock().unwrap();

    let cur = KeyboardEventSnapshot::new(state.keyboard.key, state.keyboard.modifiers);
    let mut cap = self.captured_keys.lock().unwrap();

    if Self::is_event_ready(&cur, &cap) {
      self.publish_command(&mut cap);
    } else {
      self.check_hotkeys(&cur, &mut cap);
    }
  }

  fn is_trackable_event(event: &InputEvent) -> bool {
    match event.r#type {
      EventType::KeyPress(_) | EventType::KeyRelease(_) => true,
      _ => false,
    }
  }

  fn is_event_ready(cur: &KeyboardEventSnapshot, cap: &KeyboardEventSnapshot) -> bool {
    cur.len() == 0 && cap.len() != 0
  }

  fn check_hotkeys(&self, cur: &KeyboardEventSnapshot, cap: &mut KeyboardEventSnapshot) {
    let hotkeys = self.settings.get_hotkeys();

    for hotkey in hotkeys.iter() {
      if hotkey.keys <= *cap || hotkey.keys != *cur { continue; }

      *cap = cur.clone();
    }
  }

  fn publish_command(&self, cap: &mut KeyboardEventSnapshot) {
    log!("<$>HotKeyDetector</>: Event: {:?}", cap);

    let command = CommandEvent {
      command: format!("Captured keys: {:?}", cap),
    };

    self.event_hub.publish_command(command).ok();

    *cap = KeyboardEventSnapshot::default();
  }
}
