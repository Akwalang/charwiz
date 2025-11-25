use std::sync::{Arc, Mutex};

use rust_logger::*;
use rdev::EventType;

use crate::settings::Settings;

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::common::events::{InputEvent, CommandEvent};

pub struct AutoConvertDetector {
  settings: &'static Settings,

  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,
}

impl AutoConvertDetector {
  pub fn new(settings: &'static Settings, state: Arc<Mutex<State>>, event_hub: Arc<EventHub>) -> Arc<Self> {
    Arc::new(AutoConvertDetector {
      settings,
      state: state,
      event_hub: event_hub,
    })
  }

  pub fn init(self: &Arc<Self>) {
    log!("<$>AutoConvertDetector</>: Init");

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
    let converters = self.settings.get_auto_converters();

    for converter in converters {
      if !str.ends_with(&converter.text) { continue; }

      let char_stack = state.keyboard.char_stack.clone();
      let event_stack = state.keyboard.event_stack.clone();

      let executor = converter.executor.clone();
      let injector = converter.injector.clone();

      let command = CommandEvent { char_stack, event_stack, executor, injector };

      self.event_hub.publish_command(command).ok();

      break;
    }
  }
}
