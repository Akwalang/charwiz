use std::cell::{Ref, RefCell};
use std::rc::Rc;

use rust_logger::*;
use rdev::EventType;

use tokio::sync::broadcast::error::RecvError;

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::settings::Settings;
use crate::settings::main_settings::structs::HotKey;

use crate::common::enums::UserInputCleanupEnum;
use crate::common::structs::KeyboardSnapshot;
use crate::common::events::{CommandEvent, InputEvent};

pub struct HotkeyDetector {
  settings: &'static Settings,
  
  state: Rc<RefCell<State>>,
  event_hub: Rc<EventHub>,
  captured_keys: RefCell<Option<HotKey>>,
}

impl HotkeyDetector {
  pub fn new(settings: &'static Settings, state: Rc<RefCell<State>>, event_hub: Rc<EventHub>) -> Rc<Self> {
    Rc::new(HotkeyDetector {
      settings,
      state: state,
      event_hub: event_hub,
      captured_keys: RefCell::new(None),
    })
  }

  pub fn init(self: &Rc<Self>) {
    log!("<$>Hotkey Detector</>: Init");

    self.subscribe();
  }

  fn subscribe(self: &Rc<Self>) {
    let mut input_rx = self.event_hub.input_stream();

    let this = Rc::clone(self);

    tokio::task::spawn_local(async move {
      loop {
        match input_rx.recv().await {
          Ok(event) => this.process_event(event),
          Err(RecvError::Lagged(_)) => continue,
          Err(RecvError::Closed) => break,
        }
      }
    });
  }

  fn process_event(&self, event: InputEvent) {
    if !Self::is_trackable_event(&event) { return; }
    
    let state = self.state.borrow();

    let cur = state.keyboard.get_current_snapshot();
    let mut cap = self.captured_keys.borrow_mut();

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

  fn is_event_ready(cur: &KeyboardSnapshot, cap: &Option<HotKey>) -> bool {
    let Some(hot_key) = cap else { return false; };

    cur.len() == 0 && hot_key.keys.len() != 0
  }

  fn check_hotkeys(&self, cur: &KeyboardSnapshot, cap: &mut Option<HotKey>) {
    let hotkeys = self.settings.get_hotkeys();

    for hotkey in hotkeys.iter() {
      if cap.is_some() && hotkey.keys <= cap.as_ref().unwrap().keys { continue; }
      if hotkey.keys != *cur { continue; }

      *cap = Some(hotkey.clone());
    }
  }

  fn publish_command(&self, state: Ref<'_, State>, cap: &mut Option<HotKey>) {
    let Some(hot_key) = cap else {
      error!("Unexpected empty captured hotkey");
      return;
    };

    let char_stack = state.keyboard.get_chars();
    let event_stack = state.keyboard.get_events();

    let executor = hot_key.executor.clone();
    let mut injector = hot_key.injector.clone();

    injector.user_input_cleanup = UserInputCleanupEnum::Backspace(char_stack.len() as u8);

    let command = CommandEvent { char_stack, event_stack, executor, injector };

    self.event_hub.publish_command(command).ok();

    *cap = None;
  }
}
