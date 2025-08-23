use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use rust_logger::*;
use rdev::{listen, Event, EventType, Key};

use crate::components::event_hub::{EventHub, InputEvent};
use crate::components::state::State;

use super::StickyKeys;

pub struct UserInputController {
  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,
  sticked_keys: Arc<Mutex<StickyKeys>>,
}

impl UserInputController {
  pub fn new(state: &Arc<Mutex<State>>, event_hub: &Arc<EventHub>) -> Self {
    UserInputController {
      state: state.clone(),
      event_hub: event_hub.clone(),
      sticked_keys: Arc::new(Mutex::new(StickyKeys::new())),
    }
  }

  pub fn init(&self) {
    log!("<$>UserInputController</>: Init");

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
    let state = self.state.clone();
    let sticked_keys = self.sticked_keys.clone();

    let callback = move |event: Event| {
      let mut state = state.lock().unwrap();

      if state.application.is_executing() { return; }

      if !Self::is_trackable_event(&event) { return; }
      if Self::mute_sticky_keys(&event, &sticked_keys) { return; }

      Self::write_debug_info(&event);

      state.keyboard.apply_key_event(&event);

      let event = InputEvent {
        r#type: event.event_type,
      };

      drop(state);

      if let Err(err) = hub.publish_input(event) {
        warn!("<$>UserInputController</>: Failed to publish input event: {:?}", err);
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

  fn mute_sticky_keys(event: &Event, sticked_keys: &Arc<Mutex<StickyKeys>>) -> bool {
    let key = match event.event_type {
      EventType::KeyPress(key) => key,
      EventType::KeyRelease(key) => key,
      _ => return false,
    };

    if !StickyKeys::is_sticky_key(&key) { return false; }

    let mut keys = sticked_keys.lock().unwrap();

    match event.event_type {
      EventType::KeyPress(_) => {
        let is_sticked = keys.is_pressed(&key);
        keys.add_key(&key);
        is_sticked
      },
      EventType::KeyRelease(_) => {
        let is_sticked = !keys.is_pressed(&key);
        keys.remove_key(&key);
        is_sticked
      },
      _ => false
    }
  }
}
