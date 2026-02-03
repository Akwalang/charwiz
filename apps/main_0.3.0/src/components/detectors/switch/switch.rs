use std::cell::{Ref, RefCell};
use std::rc::Rc;

use rust_logger::*;
use rdev::EventType;

use tokio::sync::broadcast::error::RecvError;

use crate::settings::Settings;

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::common::enums::{UserInputCleanupEnum, KeyboardStateCleanupEnum};
use crate::common::events::{InputEvent, CommandEvent};

use crate::settings::main_settings_structs::Switch;

pub struct SwitchDetector {
  settings: &'static Settings,

  state: Rc<RefCell<State>>,
  event_hub: Rc<EventHub>,

  captured: RefCell<Option<(Switch)>>,
}

impl SwitchDetector {
  pub fn new(settings: &'static Settings, state: Rc<RefCell<State>>, event_hub: Rc<EventHub>) -> Rc<Self> {
    Rc::new(SwitchDetector {
      settings,
      
      state,
      event_hub,

      captured: RefCell::new(None),
    })
  }

  pub fn init(self: &Rc<Self>) {
    log!("<$>Switch Detector</>: Init");

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

  fn process_event(self: &Rc<Self>, event: InputEvent) {
    if !Self::is_trackable_event(&event) { return; }

    let state = self.state.borrow();

    let input = state.keyboard.get_string();
    let snapshot = state.keyboard.get_current_snapshot();

    let mut captured = self.captured.borrow_mut();

    if captured.is_some() && snapshot.len() == 0 {
      self.publish_command(state, captured.as_ref().unwrap().clone());
      *captured = None;

      return;
    }

    let switch = self.find_switch(&input);

    if switch.is_some() {
      *captured = switch;
    }
  }

  fn is_trackable_event(event: &InputEvent) -> bool {
    match event.r#type {
      EventType::KeyPress(_) | EventType::KeyRelease(_) => true,
      _ => false,
    }
  }

  fn find_switch(&self, input: &str) -> Option<Switch> {
    let switches = self.settings.get_switches();

    for switch in switches.iter() {
      if input.ends_with(&switch.text) { 
        return Some(switch.clone());
      }
    }

    None
  }

  fn publish_command(&self, state: Ref<'_, State>, switch: Switch) {
      let current_snapshot = state.keyboard.get_current_snapshot();

      let char_stack = state.keyboard.get_chars();
      let event_stack = state.keyboard.get_events();

      let executor = switch.executor.clone();
      let mut injector = switch.injector.clone();

      injector.user_input_cleanup = UserInputCleanupEnum::Backspace(switch.text.chars().count() as u8);
      injector.keyboard_state_cleanup = KeyboardStateCleanupEnum::Drop;

      let command = CommandEvent { current_snapshot, char_stack, event_stack, executor, injector };

      self.event_hub.publish_command(command).ok();
  }
}
