use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[cfg(feature = "logger")]
use logger::*;

use rdev::EventType;

use tokio::sync::broadcast::error::RecvError;

use settings_core::enums::UserInputCleanupEnum;
use settings_core::structs::KeyboardSnapshot;
use settings_core::settings::main_settings::HotKey;

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::settings::Settings;

use crate::common::events::{CommandEvent, InputEvent};

pub struct HotkeyDetector {
  settings: &'static Settings,
  
  state: Rc<RefCell<State>>,
  event_hub: Rc<EventHub>,

  captured: RefCell<Option<HotKey>>,
}

impl HotkeyDetector {
  pub fn new(settings: &'static Settings, state: Rc<RefCell<State>>, event_hub: Rc<EventHub>) -> Rc<Self> {
    Rc::new(HotkeyDetector {
      settings,
      state: state,
      event_hub: event_hub,
      captured: RefCell::new(None),
    })
  }

  pub fn init(self: &Rc<Self>) {
    #[cfg(feature = "logger")]
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

    let snapshot = state.keyboard.get_current_snapshot();
    let mut captured = self.captured.borrow_mut();

    if captured.is_some() && snapshot == KeyboardSnapshot::default() {
      self.publish_command(state, captured.as_ref().unwrap().clone());
      *captured = None;

      return;
    }

    let Some(hotkey) = self.find_hotkey(&snapshot) else {
      return;
    };

    if captured.is_none() || hotkey.keys >= captured.as_ref().unwrap().keys {
      *captured = Some(hotkey);
    }
  }

  fn is_trackable_event(event: &InputEvent) -> bool {
    match event.r#type {
      EventType::KeyPress(_) | EventType::KeyRelease(_) => true,
      _ => false,
    }
  }

  fn find_hotkey(&self, snapshot: &KeyboardSnapshot) -> Option<HotKey> {
    let hotkeys = self.settings.get_hotkeys();

    for hotkey in hotkeys.iter() {
      if *snapshot == hotkey.keys { 
        return Some(hotkey.clone());
      }
    }

    None
  }

  fn publish_command(&self, state: Ref<'_, State>, hotkey: HotKey) {
    let char_stack = state.keyboard.get_chars();
    let event_stack = state.keyboard.get_events();

    let executor = hotkey.executor.clone();
    let mut injector = hotkey.injector.clone();

    injector.user_input_cleanup = UserInputCleanupEnum::Backspace(char_stack.len() as u8);

    let command = CommandEvent { char_stack, event_stack, executor, injector };

    self.event_hub.publish_command(command).ok();
  }
}
