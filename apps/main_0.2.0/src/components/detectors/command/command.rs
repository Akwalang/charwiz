use std::sync::{Arc, Mutex};

use rust_logger::*;
use rdev::EventType;

use crate::settings::Settings;

use crate::components::event_hub::{EventHub, InputEvent, CommandEvent};
use crate::components::state::State;

pub struct CommandDetector {
  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,
}

impl CommandDetector {
  pub fn new(state: &Arc<Mutex<State>>, event_hub: &Arc<EventHub>) -> Arc<Self> {
    Arc::new(CommandDetector {
      state: state.clone(),
      event_hub: event_hub.clone(),
    })
  }

  pub fn init(self: &Arc<Self>) {
    log!("<$>CommandDetector</>: Init");

    self.subscribe();
  }

  fn subscribe(self: &Arc<Self>) {
    let mut input_rx = self.event_hub.input_stream();

    let this = Arc::clone(self);

    async_std::task::spawn(async move {
      while let Ok(event) = input_rx.recv().await {
        this.process_event(event);
      }
    });
  }

  fn process_event(self: &Arc<Self>, event: InputEvent) {
    let state = self.state.lock().unwrap();

    let EventType::KeyPress(_) = event.r#type else { return; };

    let str = state.keyboard.get_string();
    let commands = Settings::get_commands();

    for command in commands {
      if str.ends_with(&command) {
        let command = CommandEvent {
          command: format!("Captured command: <!>{}</>", str),
        };

        self.event_hub.publish_command(command).ok();
      }
    }
  }
}
