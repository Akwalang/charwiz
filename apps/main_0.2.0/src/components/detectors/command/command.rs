use std::sync::{Arc, Mutex};
use std::collections::HashSet;

use rust_logger::*;

use rdev::{EventType, Key};

use crate::platform::Platform;
use crate::settings::Settings;

use crate::components::event_hub::{EventHub, InputEvent};

pub struct CommandDetector {
  event_hub: Arc<EventHub>,
  modifiers: Mutex<HashSet<Key>>,
  stack: Vec<InputEvent>,
}

impl CommandDetector {
  pub fn new(event_hub: &Arc<EventHub>) -> Arc<Self> {
    let modifiers = HashSet::with_capacity(6);

    let this = CommandDetector {
      event_hub: event_hub.clone(),
      modifiers: Mutex::new(modifiers),
      stack: Vec::new(),
    };

    Arc::new(this)
  }

  pub fn init(self: &Arc<Self>) {
    log!("<purple>CommandDetector</>: Init");

    self.subscribe();
  }

  fn subscribe(self: &Arc<Self>) {
    let input_rx = self.event_hub.input_stream();

    let this = Arc::clone(self);

    async_std::task::spawn(async move {
      let mut input_rx = input_rx;

      while let Ok(event) = input_rx.recv().await {
        let this = Arc::clone(&this);
      
        this.process_event(event);
      }
    });
  }

  fn process_event(self: &Arc<Self>, event: InputEvent) {
    match event.r#type {
      EventType::KeyPress(_) => self.process_keyboard_event(event),
      EventType::KeyRelease(_) => self.process_keyboard_event(event),
      _ => {},
    }
  }

  fn process_keyboard_event(self: &Arc<Self>, event: InputEvent) {
    let Some((key, state)) = Self::get_key(&event) else { return; };

    if Self::is_modifier(&key) {
      self.handle_modifier_update(key, state);
    } else if state {
      self.handle_insert(key);
    }
  }

  fn handle_modifier_update(self: &Arc<Self>, key: Key, state: bool) {
    let mut modifiers = self.modifiers.lock().unwrap();

    if state {
      modifiers.insert(key);
    } else {
      modifiers.remove(&key);
    }
  }

  fn handle_insert(self: &Arc<Self>, key: Key) {
    let modifiers = self.modifiers.lock().unwrap();

    let settings_kl = Settings::get_keyboard_layouts();
    let platform_kl = Platform::get_keyboard_layouts();

    let cur_layout = platform_kl.get_current_keyboard_layout();

    let (char, is_exists) = settings_kl.find_combination(&cur_layout.name, &key, &modifiers);

    println!("Char: {:?}, Is Combination Exists: {}", char, is_exists);
  }

  fn get_key(event: &InputEvent) -> Option<(Key, bool)> {
    match event.r#type {
      EventType::KeyPress(key) => Some((key, true)),
      EventType::KeyRelease(key) => Some((key, false)),
      _ => None,
    }
  }

  fn is_modifier(key: &Key) -> bool {
    match key {
      Key::Alt => true,
      Key::AltGr => true,
      Key::ShiftLeft => true,
      Key::ShiftRight => true,
      Key::ControlLeft => true,
      Key::ControlRight => true,
      _ => false,
    }
  }
}
