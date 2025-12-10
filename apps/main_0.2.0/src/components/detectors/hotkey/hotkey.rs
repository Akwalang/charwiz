use std::sync::{Arc, Mutex, MutexGuard};

use rust_logger::*;
use rdev::EventType;

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::settings::{Settings, structs::HotKey};

use crate::common::enums::UserInputCleanupEnum;
use crate::common::events::{CommandEvent, InputEvent};
use crate::common::structs::KeyboardEventSnapshot;

pub struct HotkeyDetector {
  settings: &'static Settings,
  
  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,
  captured_keys: Mutex<Option<HotKey>>,
}

impl HotkeyDetector {
  pub fn new(settings: &'static Settings, state: Arc<Mutex<State>>, event_hub: Arc<EventHub>) -> Arc<Self> {
    Arc::new(HotkeyDetector {
      settings,
      state: state,
      event_hub: event_hub,
      captured_keys: Mutex::new(None),
    })
  }

  pub fn init(self: &Arc<Self>) {
    log!("<$>HotKeyDetector</>: Init");

    self.subscribe();
  }

  fn subscribe(self: &Arc<Self>) {
    let mut input_rx = self.event_hub.input_stream();

    let this = Arc::clone(self);

    tokio::task::spawn_local(async move {
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
      self.publish_command(state, &mut cap);
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

  fn is_event_ready(cur: &KeyboardEventSnapshot, cap: &Option<HotKey>) -> bool {
    let Some(hot_key) = cap else { return false; };

    cur.len() == 0 && hot_key.keys.len() != 0
  }

  fn check_hotkeys(&self, cur: &KeyboardEventSnapshot, cap: &mut Option<HotKey>) {
    let hotkeys = self.settings.get_hotkeys();

    for hotkey in hotkeys.iter() {
      if cap.is_some() && hotkey.keys <= cap.as_ref().unwrap().keys { continue; }
      if hotkey.keys != *cur { continue; }

      *cap = Some(hotkey.clone());
    }
  }

  fn publish_command(&self, state: MutexGuard<'_, State>, cap: &mut Option<HotKey>) {
    let Some(hot_key) = cap else {
      error!("Unexpected empty captured hotkey");
      return;
    };

    let char_stack = state.keyboard.char_stack.clone();
    let event_stack = state.keyboard.event_stack.clone();

    let executor = hot_key.executor.clone();
    let mut injector = hot_key.injector.clone();

    injector.user_input_cleanup = UserInputCleanupEnum::Backspace(char_stack.len() as u8);

    let command = CommandEvent { char_stack, event_stack, executor, injector };

    self.event_hub.publish_command(command).ok();

    *cap = None;
  }
}
