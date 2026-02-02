use std::cell::RefCell;
use std::rc::Rc;

use rust_logger::*;
use rdev::EventType;

use crate::settings::Settings;

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::common::enums::{UserInputCleanupEnum, KeyboardStateCleanupEnum};
use crate::common::events::{CommandEvent, InputEvent};

pub struct CommandDetector {
  settings: &'static Settings,

  state: Rc<RefCell<State>>,
  event_hub: Rc<EventHub>,
}

impl CommandDetector {
  pub fn new(settings: &'static Settings, state: Rc<RefCell<State>>, event_hub: Rc<EventHub>) -> Rc<Self> {
    Rc::new(CommandDetector {
      settings,
      state: state,
      event_hub: event_hub,
    })
  }

  pub fn init(self: &Rc<Self>) {
    log!("<$>Command Detector</>: Init");

    self.subscribe();
  }

  fn subscribe(self: &Rc<Self>) {
    let mut input_rx = self.event_hub.input_stream();

    let this = Rc::clone(self);

    tokio::task::spawn_local(async move {
      while let Ok(event) = input_rx.recv().await {
        this.process_event(event);
      }
    });
  }

  fn process_event(self: &Rc<Self>, event: InputEvent) {
    let state = self.state.borrow();

    let EventType::KeyPress(_) = event.r#type else { return; };

    let str = state.keyboard.get_string();
    let commands = self.settings.get_commands();

    for command in commands.iter() {
      if !str.ends_with(&command.cmd) { continue; }

      let char_stack = state.keyboard.get_chars();
      let event_stack = state.keyboard.get_events();

      let executor = command.executor.clone();
      let mut injector = command.injector.clone();

      injector.user_input_cleanup = UserInputCleanupEnum::Backspace(command.cmd.chars().count() as u8);
      injector.keyboard_state_cleanup = KeyboardStateCleanupEnum::Drop;

      let command = CommandEvent { char_stack, event_stack, executor, injector };

      self.event_hub.publish_command(command).ok();

      break;
    }
  }
}
