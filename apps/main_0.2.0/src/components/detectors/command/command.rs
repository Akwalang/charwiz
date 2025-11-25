use std::sync::{Arc, Mutex};

use rust_logger::*;
use rdev::EventType;

use crate::settings::Settings;

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::common::events::{CommandEvent, InputEvent};

pub struct CommandDetector {
  settings: &'static Settings,

  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,
}

impl CommandDetector {
  pub fn new(settings: &'static Settings, state: Arc<Mutex<State>>, event_hub: Arc<EventHub>) -> Arc<Self> {
    Arc::new(CommandDetector {
      settings,
      state: state,
      event_hub: event_hub,
    })
  }

  pub fn init(self: &Arc<Self>) {
    log!("<$>CommandDetector</>: Init");

    self.subscribe();
  }

  fn subscribe(self: &Arc<Self>) {
    let mut input_rx = self.event_hub.input_stream();

    let this = Arc::clone(self);

    tokio::task::spawn_local(async move {
      while let Ok(event) = input_rx.recv().await {
        this.process_event(event);
      }
    });
  }

  fn process_event(self: &Arc<Self>, event: InputEvent) {
    let state = self.state.lock().unwrap();

    let EventType::KeyPress(_) = event.r#type else { return; };

    let str = state.keyboard.get_string();
    let commands = self.settings.get_commands();

    for command in commands {
      if !str.ends_with(&command.cmd) { continue; }

      let executor = command.executor.clone();
      let injector = command.injector.clone();

      let command = CommandEvent { executor, injector };

      self.event_hub.publish_command(command).ok();

      break;
    }
  }
}
