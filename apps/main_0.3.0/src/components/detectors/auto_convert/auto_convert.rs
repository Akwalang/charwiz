use std::cell::RefCell;
use std::rc::Rc;

use rust_logger::*;
use rdev::EventType;

use crate::settings::Settings;

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::common::enums::{UserInputCleanupEnum, KeyboardStateCleanupEnum};
use crate::common::events::{InputEvent, CommandEvent};

pub struct AutoConvertDetector {
  settings: &'static Settings,

  state: Rc<RefCell<State>>,
  event_hub: Rc<EventHub>,
}

impl AutoConvertDetector {
  pub fn new(settings: &'static Settings, state: Rc<RefCell<State>>, event_hub: Rc<EventHub>) -> Rc<Self> {
    Rc::new(AutoConvertDetector {
      settings,
      state,
      event_hub,
    })
  }

  pub fn init(self: &Rc<Self>) {
    log!("<$>Auto Convert Detector</>: Init");

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
    let converters = self.settings.get_auto_converters();

    for converter in converters.iter() {
      if !str.ends_with(&converter.text) { continue; }

      let char_stack = state.keyboard.get_chars();
      let event_stack = state.keyboard.get_events();

      let executor = converter.executor.clone();
      let mut injector = converter.injector.clone();

      injector.user_input_cleanup = UserInputCleanupEnum::Backspace(converter.text.chars().count() as u8);
      injector.keyboard_state_cleanup = KeyboardStateCleanupEnum::Drop;

      let command = CommandEvent { char_stack, event_stack, executor, injector };

      self.event_hub.publish_command(command).ok();

      break;
    }
  }
}
