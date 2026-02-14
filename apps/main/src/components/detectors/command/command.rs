use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[cfg(feature = "logger")]
use logger::*;

use rdev::EventType;

use tokio::sync::broadcast::error::RecvError;

use settings_core::enums::{UserInputCleanupEnum, KeyboardStateCleanupEnum};
use settings_core::structs::KeyboardSnapshot;
use settings_core::settings::main_settings::Command;

use crate::settings::Settings;

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::common::events::{CommandEvent, InputEvent};

pub struct CommandDetector {
  settings: &'static Settings,

  state: Rc<RefCell<State>>,
  event_hub: Rc<EventHub>,

  captured: RefCell<Option<Command>>,
}

impl CommandDetector {
  pub fn new(settings: &'static Settings, state: Rc<RefCell<State>>, event_hub: Rc<EventHub>) -> Rc<Self> {
    Rc::new(CommandDetector {
      settings,

      state: state,
      event_hub: event_hub,
      
      captured: RefCell::new(None),
    })
  }

  pub fn init(self: &Rc<Self>) {
    #[cfg(feature = "logger")]
    log!("<$>Command Detector</>: Init");

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

    let input = state.keyboard.get_char_stack();
    let snapshot = state.keyboard.get_current_snapshot();

    let mut captured = self.captured.borrow_mut();

    if captured.is_some() && snapshot == KeyboardSnapshot::default() {
      self.publish_command(state, captured.as_ref().unwrap().clone());
      *captured = None;

      return;
    }

    *captured = self.find_command(input);
  }

  fn is_trackable_event(event: &InputEvent) -> bool {
    match event.r#type {
      EventType::KeyPress(_) | EventType::KeyRelease(_) => true,
      _ => false,
    }
  }

  fn find_command(&self, input: &Vec<char>) -> Option<Command> {
    let commands = self.settings.get_commands();

    for command in commands.iter() {
      if input.ends_with(&command.cmd) { 
        return Some(command.clone());
      }
    }

    None
  }

  fn publish_command(&self, state: Ref<'_, State>, command: Command) {
    let char_stack = state.keyboard.get_chars();
    let event_stack = state.keyboard.get_events();

    let transformer = command.transformer.clone();
    let mut injector = command.injector.clone();

    injector.user_input_cleanup = UserInputCleanupEnum::Backspace(command.cmd.len() as u8);
    injector.keyboard_state_cleanup = KeyboardStateCleanupEnum::Drop;

    let command = CommandEvent { char_stack, event_stack, transformer, injector };

    self.event_hub.publish_command(command).ok();
  }
}
