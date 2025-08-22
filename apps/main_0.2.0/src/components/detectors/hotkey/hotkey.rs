use std::sync::{Arc, Mutex};
use std::collections::HashSet;

use rust_logger::*;
use rdev::{EventType, Key};

use crate::components::event_hub::{EventHub, InputEvent, CommandEvent};
use crate::components::state::State;

use crate::settings::Settings;

pub struct HotkeyDetector {
  state: Arc<State>,
  event_hub: Arc<EventHub>,
  current_keys: Mutex<HashSet<Key>>,
  captured_keys: Mutex<HashSet<Key>>,
}

impl HotkeyDetector {
  pub fn new(state: &Arc<State>, event_hub: &Arc<EventHub>) -> Arc<Self> {
    Arc::new(HotkeyDetector {
      state: state.clone(),
      event_hub: event_hub.clone(),
      current_keys: Mutex::new(HashSet::new()),
      captured_keys: Mutex::new(HashSet::new()),
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
    
    let mut cur = self.current_keys.lock().unwrap();
    let mut cap = self.captured_keys.lock().unwrap();

    match event.r#type {
      EventType::KeyPress(key) => { cur.insert(key); },
      EventType::KeyRelease(key) => { cur.remove(&key); },
      _ => {},
    }

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

  fn is_event_ready(cur: &HashSet<Key>, cap: &HashSet<Key>) -> bool {
    cur.len() == 0 && cap.len() != 0
  }

  fn check_hotkeys(&self, cur: &HashSet<Key>, cap: &mut HashSet<Key>) {
    let hotkeys = Settings::get_hotkeys();

    for keys in hotkeys.iter() {
      if keys.len() <= cap.len() { continue; }
      if !keys.eq(cur) { continue; }

      cap.clear();
      cap.extend(cur.iter().cloned());
    }
  }

  fn publish_command(&self, cap: &mut HashSet<Key>) {
    let keys = cap.iter()
      .map(|k| format!("<!>{:?}</>", k))
      .collect::<Vec<_>>()
      .join(", ");

    log!("<$>HotKeyDetector</>: Event: {}", keys);

    let command = CommandEvent {
      command: format!("Captured keys: {}", keys),
    };

    self.event_hub.publish_command(command).ok();

    cap.clear();
  }
}
