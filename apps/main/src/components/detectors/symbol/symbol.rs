use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[cfg(feature = "logger")]
use logger::*;

use rdev::EventType;

use tokio::sync::broadcast::error::RecvError;

use crate::common::enums::UserInputCleanupEnum;
use crate::common::events::{CommandEvent, InputEvent};
use crate::common::structs::KeyboardSnapshot;

use crate::components::event_hub::EventHub;
use crate::components::state::State;

use crate::settings::Settings;
use crate::settings::main_settings_structs::Symbol;

pub struct SymbolDetector {
  settings: &'static Settings,
  
  state: Rc<RefCell<State>>,
  event_hub: Rc<EventHub>,

  captured: RefCell<Option<Symbol>>,
}

impl SymbolDetector {
  pub fn new(settings: &'static Settings, state: Rc<RefCell<State>>, event_hub: Rc<EventHub>) -> Rc<Self> {
    Rc::new(SymbolDetector {
      settings,
      
      state,
      event_hub,

      captured: RefCell::new(None),
    })
  }

  pub fn init(self: &Rc<Self>) {
    #[cfg(feature = "logger")]
    log!("<$>Symbol Detector</>: Init");

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
    let mut captured = self.captured.borrow_mut();

    let snapshot = state.keyboard.get_current_snapshot();

    if let Some(symbol) = captured.as_ref() {
      if snapshot == KeyboardSnapshot::default() {
        self.publish_command(state, symbol.clone());
        *captured = None;
      }

      return;
    }

    *captured = self.find_symbol(&snapshot);
  }

  fn is_trackable_event(event: &InputEvent) -> bool {
    match event.r#type {
      EventType::KeyPress(_) | EventType::KeyRelease(_) => true,
      _ => false,
    }
  }

  fn find_symbol(&self, snapshot: &KeyboardSnapshot) -> Option<Symbol> {
    let symbols = self.settings.get_symbols();

    for symbol in symbols.iter() {
      if *snapshot == symbol.keys { 
        return Some(symbol.clone());
      }
    }

    None
  }

  fn publish_command(&self, state: Ref<'_, State>, symbol: Symbol) {
    let current_snapshot = state.keyboard.get_current_snapshot();

    let char_stack = state.keyboard.get_chars();
    let event_stack = state.keyboard.get_events();

    let executor = symbol.executor.clone();
    let mut injector = symbol.injector.clone();

    injector.user_input_cleanup = UserInputCleanupEnum::None;

    let command = CommandEvent { current_snapshot, char_stack, event_stack, executor, injector };

    self.event_hub.publish_command(command).ok();
  }
}
