use std::sync::{Arc, Mutex, MutexGuard};

use tokio::sync::mpsc;

use rust_logger::*;
use rdev::{listen, Event, EventType};

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::common::events::InputEvent;

use super::StickyKeys;

pub struct UserInputController {
  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,
  sticked_keys: Arc<Mutex<StickyKeys>>,
}

impl UserInputController {
  pub fn new(state: &Arc<Mutex<State>>, event_hub: &Arc<EventHub>) -> Arc<Self> {
    Arc::new(UserInputController {
      state: state.clone(),
      event_hub: event_hub.clone(),
      sticked_keys: Arc::new(Mutex::new(StickyKeys::new())),
    })
  }

  pub fn init(self: &Arc<Self>) {
    log!("<$>UserInputController</>: Init");

    self.subscribe();
    self.listen();
  }

  fn subscribe(self: &Arc<Self>) {
    let mut status_rx = self.event_hub.status_stream();

    tokio::task::spawn_local(async move {
      while let Ok(event) = status_rx.recv().await {
        log!("Received status event: {:?}", event);
      }
    });
  }

  fn listen(self: &Arc<Self>) {
    let (tx, mut rx) = mpsc::unbounded_channel();

    let this = Arc::clone(&self);

    tokio::task::spawn_local(async move {
      while let Some(event) = rx.recv().await {
        this.process_event(event);
      }
    });

    tokio::task::spawn_blocking(move || {
      let callback = move |event: Event| {
        let _ = tx.send(event);
      };

      if let Err(error) = listen(callback) {
        error!("<$>UserInputController</>: Can't start listen keyboard events. Error: {:?}", error);
      }
    });
  }

  fn process_event(&self, event: Event) {
    let mut state = self.state.lock().unwrap();
    let mut sticked_keys = self.sticked_keys.lock().unwrap();

    if !Self::is_trackable_event(&event) { return; }
    if Self::mute_sticky_keys(&event, &mut sticked_keys) { return; }

    let is_executing = state.application.is_executing();

    Self::write_debug_info(&event, is_executing);

    if is_executing { return; }

    state.keyboard.apply_key_event(&event);

    let event = InputEvent {
      r#type: event.event_type,
    };

    drop(state);

    if let Err(err) = self.event_hub.publish_input(event) {
      warn!("<$>UserInputController</>: Failed to publish input event: {:?}", err);
    }
  }

  fn is_trackable_event(event: &Event) -> bool {
    match event.event_type {
      EventType::KeyPress(_) | EventType::KeyRelease(_) => true,
      EventType::ButtonPress(_) | EventType::ButtonRelease(_) => true,
      _ => false,
    }
  }

  fn write_debug_info(event: &Event, is_muted: bool) {
    let stl = if is_muted { "i" } else { "i!" };

    match event.event_type {
      EventType::KeyPress(key) => { debug!("Key press: <{}>{:?}</>", stl, key); },
      EventType::KeyRelease(key) => { debug!("Key release: <{}>{:?}</>", stl, key); },
      EventType::ButtonPress(button) => { debug!("Mouse press: <{}>{:?}</>", stl, button); },
      EventType::ButtonRelease(button) => { debug!("Mouse release: <{}>{:?}</>", stl, button); },
      _ => {},
    }
  }

  fn mute_sticky_keys(event: &Event, sticked_keys: &mut MutexGuard<'_, StickyKeys>) -> bool {
    let key = match event.event_type {
      EventType::KeyPress(key) => key,
      EventType::KeyRelease(key) => key,
      _ => return false,
    };

    if !StickyKeys::is_sticky_key(&key) { return false; }

    match event.event_type {
      EventType::KeyPress(_) => {
        let is_sticked = sticked_keys.is_pressed(&key);
        sticked_keys.add_key(&key);
        is_sticked
      },
      EventType::KeyRelease(_) => {
        let is_sticked = !sticked_keys.is_pressed(&key);
        sticked_keys.remove_key(&key);
        is_sticked
      },
      _ => false
    }
  }
}
