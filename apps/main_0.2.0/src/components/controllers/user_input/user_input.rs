use std::collections::HashSet;
use std::sync::{Arc, Mutex, atomic::AtomicBool};

use rust_logger::*;

use rdev::{listen, Event, EventType, Key};

use crate::components::event_hub::{EventHub, InputEvent};

const STICKY_KEYS: [Key; 16] = [
  Key::MetaLeft,
  Key::MetaRight,
  Key::ControlLeft,
  Key::ControlRight,
  Key::ShiftLeft,
  Key::ShiftRight,
  Key::Alt,
  Key::AltGr,
  Key::UpArrow,
  Key::DownArrow,
  Key::LeftArrow,
  Key::RightArrow,
  Key::Home,
  Key::End,
  Key::PageUp,
  Key::PageDown,
];

pub struct UserInputController {
  event_hub: Arc<EventHub>,
  locked: Arc<AtomicBool>,
  sticked_keys: Arc<Mutex<HashSet<Key>>>,
}

impl UserInputController {
  pub fn new(event_hub: &Arc<EventHub>) -> Self {
    UserInputController {
      event_hub: event_hub.clone(),
      locked: Arc::new(AtomicBool::new(false)),
      sticked_keys: Arc::new(Mutex::new(HashSet::new())),
    }
  }

  pub fn init(&self) {
    log!("<purple>UserInputController</>: Init");

    self.subscribe();
    self.listen();
  }

  fn subscribe(&self) {
    let mut status_rx = self.event_hub.status_stream();

    async_std::task::spawn(async move {
      while let Ok(event) = status_rx.recv().await {
        log!("Received status event: {:?}", event);
      }
    });
  }

  fn listen(&self) {
    let hub = self.event_hub.clone();
    let locked = self.locked.clone();
    let sticked_keys = self.sticked_keys.clone();

    let callback = move |event: Event| {
      if locked.load(std::sync::atomic::Ordering::SeqCst) { return; }
      if !Self::is_trackable_event(&event) { return; }
      if Self::mute_sticky_keys(&event, &sticked_keys) { return; }

      Self::write_debug_info(&event);

      let event = InputEvent {
        r#type: event.event_type,
      };

      if let Err(err) = hub.publish_input(event) {
        warn!("Failed to publish input event: {:?}", err);
      }
    };

    async_std::task::spawn(async move {
      if let Err(error) = listen(callback) {
        panic!("Can't start listen keyboard events.\nError: {:?}", error);
      }
    });
  }

  fn is_trackable_event(event: &Event) -> bool {
    match event.event_type {
      EventType::KeyPress(_) | EventType::KeyRelease(_) => true,
      EventType::ButtonPress(_) | EventType::ButtonRelease(_) => true,
      _ => false,
    }
  }

  fn write_debug_info(event: &Event) {
    match event.event_type {
      EventType::KeyPress(key) => { debug!("Key press: {:?}", key); },
      EventType::KeyRelease(key) => { debug!("Key release: {:?}", key); },
      EventType::ButtonPress(button) => { debug!("Mouse press: {:?}", button); },
      EventType::ButtonRelease(button) => { debug!("Mouse release: {:?}", button); },
      _ => {},
    }
  }

  fn mute_sticky_keys(event: &Event, sticked_keys: &Arc<Mutex<HashSet<Key>>>) -> bool {
    let sticked_keys = sticked_keys.clone();

    let key = match event.event_type {
      EventType::KeyPress(key) => key,
      EventType::KeyRelease(key) => key,
      _ => return false,
    };

    if !STICKY_KEYS.contains(&key) { return false; }

    let mut keys = sticked_keys.lock().unwrap();

    match event.event_type {
      EventType::KeyPress(_) => {
        let is_sticked = keys.contains(&key);
        keys.insert(key);
        is_sticked
      },
      EventType::KeyRelease(_) => {
        let is_sticked = !keys.contains(&key);
        keys.remove(&key);
        is_sticked
      },
      _ => false
    }
  }
}
