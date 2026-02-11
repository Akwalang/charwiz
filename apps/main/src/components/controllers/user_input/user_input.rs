use std::cell::RefCell;
use std::rc::Rc;

use tokio::sync::mpsc;

#[cfg(feature = "logger")]
use logger::*;

use rdev::{listen, Event, EventType};

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::common::events::InputEvent;

use super::StickyKeys;

pub struct UserInputController {
  state: Rc<RefCell<State>>,
  event_hub: Rc<EventHub>,
  sticked_keys: Rc<RefCell<StickyKeys>>,
}

impl UserInputController {
  pub fn new(state: Rc<RefCell<State>>, event_hub: Rc<EventHub>) -> Rc<Self> {
    Rc::new(UserInputController {
      state: state.clone(),
      event_hub: event_hub.clone(),
      sticked_keys: Rc::new(RefCell::new(StickyKeys::new())),
    })
  }

  pub fn init(self: &Rc<Self>) {
    #[cfg(feature = "logger")]
    log!("<$>User Input Controller</>: Init");

    self.listen();
  }

  fn listen(self: &Rc<Self>) {
    let (tx, mut rx) = mpsc::unbounded_channel();

    let this = Rc::clone(&self);

    tokio::task::spawn_local(async move {
      while let Some(event) = rx.recv().await {
        this.process_event(event);
      }
    });

    tokio::task::spawn_blocking(move || {
      let callback = move |event: Event| {
        let _ = tx.send(event);
      };

      if let Err(_error) = listen(callback) {
        #[cfg(feature = "logger")]
        error!("<$>User Input Controller</>: Can't start listen keyboard events. Error: {:?}", _error);
      }
    });
  }

  fn process_event(&self, event: Event) {
    let mut state = self.state.borrow_mut();
    let mut sticked_keys = self.sticked_keys.borrow_mut();

    if state.application.is_disabled() { return; }

    if !Self::is_trackable_event(&event) { return; }
    if Self::mute_sticky_keys(&event, &mut sticked_keys) { return; }

    let is_executing = state.application.is_executing();

    #[cfg(feature = "logger")]
    Self::write_debug_info(&event, is_executing);

    state.keyboard.apply_key_event(&event);

    if is_executing { return; }

    let event = InputEvent {
      r#type: event.event_type,
    };

    drop(state);

    if let Err(_error) = self.event_hub.publish_input(event) {
      #[cfg(feature = "logger")]
      warn!("<$>User Input Controller</>: Failed to publish input event: {:?}", _error);
    }
  }

  fn is_trackable_event(event: &Event) -> bool {
    match event.event_type {
      EventType::KeyPress(_) | EventType::KeyRelease(_) => true,
      EventType::ButtonPress(_) | EventType::ButtonRelease(_) => true,
      _ => false,
    }
  }

  #[cfg(feature = "logger")]
  fn write_debug_info(event: &Event, is_muted: bool) {
    let stl = if is_muted { "i" } else { "i!" };

    match event.event_type {
      EventType::KeyPress(key)         => debug!("Key press: <{}>{:?}</>", stl, key),
      EventType::KeyRelease(key)       => debug!("Key release: <{}>{:?}</>", stl, key),
      EventType::ButtonPress(button)   => debug!("Mouse press: <{}>{:?}</>", stl, button),
      EventType::ButtonRelease(button) => debug!("Mouse release: <{}>{:?}</>", stl, button),
      _ => {},
    }
  }

  fn mute_sticky_keys(event: &Event, sticked_keys: &mut StickyKeys) -> bool {
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
