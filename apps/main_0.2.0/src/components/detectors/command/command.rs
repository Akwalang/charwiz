use std::sync::{Arc, Mutex};
use std::collections::HashSet;

use rust_logger::*;
use rdev::{EventType, Key};

use crate::platform::Platform;
use crate::settings::Settings;

use crate::components::event_hub::{EventHub, InputEvent};
use crate::components::executor::PrintCharCommand;
use crate::components::state::State;

use crate::common::structs::KeyboardModifiers;

pub struct CommandDetector {
  state: Arc<State>,
  event_hub: Arc<EventHub>,
  modifiers: Mutex<KeyboardModifiers>,
  char_stack: Mutex<Vec<char>>,
  print_stack: Mutex<Vec<PrintCharCommand>>,
}

impl CommandDetector {
  pub fn new(state: &Arc<State>, event_hub: &Arc<EventHub>) -> Arc<Self> {
    let modifiers = KeyboardModifiers::new();

    let this = CommandDetector {
      state: state.clone(),
      event_hub: event_hub.clone(),
      modifiers: Mutex::new(modifiers),
      char_stack: Mutex::new(Vec::with_capacity(10)),
      print_stack: Mutex::new(Vec::with_capacity(10)),
    };

    Arc::new(this)
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
    match event.r#type {
      EventType::ButtonPress(_) | EventType::ButtonRelease(_) => self.process_mouse_event(event),
      EventType::KeyPress(_) | EventType::KeyRelease(_) => self.process_keyboard_event(event),
      _ => {},
    }
  }

  fn process_mouse_event(self: &Arc<Self>, event: InputEvent) {
    let EventType::ButtonPress(_) = event.r#type else { return; };

    self.stack_clear();
  }

  fn process_keyboard_event(self: &Arc<Self>, event: InputEvent) {
    let Some((key, state)) = Self::get_key(&event) else { return; };

    if Self::is_modifier(&key) {
      self.handle_modifier_update(key, state);
    }
    
    if !state { return; }
    
    if Self::is_backspace(&key) {
      self.handle_backspace(key);
    } else if self.is_stack_breaker(&key) {
      self.handle_stack_breaker(key);
    } else {
      self.handle_insert(key);
    }
  }

  fn handle_modifier_update(self: &Arc<Self>, key: Key, state: bool) {
    let mut modifiers = self.modifiers.lock().unwrap();

    if state {
      modifiers.add_key(&key);
    } else {
      modifiers.remove_key(&key);
    }
  }

  fn handle_backspace(self: &Arc<Self>, _: Key) {
    self.stack_pop();
  }

  fn handle_stack_breaker(self: &Arc<Self>, _: Key) {
    self.stack_clear();
  }

  fn handle_insert(self: &Arc<Self>, key: Key) {
    let modifiers = self.modifiers.lock().unwrap();

    let mut char_stack = self.char_stack.lock().unwrap();
    let mut print_stack = self.print_stack.lock().unwrap();

    let settings_kl = Settings::get_keyboard_layouts();
    let platform_kl = Platform::get_keyboard_layouts();

    let cur_layout = platform_kl.get_current_keyboard_layout();

    let (char, is_exists) = settings_kl.find_combination(&cur_layout.name, &key, *modifiers);

    if !is_exists { return; }

    if let Some(char) = char {
      char_stack.push(char);
    }

    print_stack.push(Self::create_print_command(&key, *modifiers));

    println!("Stack: {:?}", char_stack.iter().collect::<String>());
  }

  fn stack_pop(self: &Arc<Self>) {
    self.char_stack.lock().unwrap().pop();
    self.print_stack.lock().unwrap().pop();
  }

  fn stack_clear(self: &Arc<Self>) {
    self.char_stack.lock().unwrap().clear();
    self.print_stack.lock().unwrap().clear();
  }

  fn get_key(event: &InputEvent) -> Option<(Key, bool)> {
    match event.r#type {
      EventType::KeyPress(key) => Some((key, true)),
      EventType::KeyRelease(key) => Some((key, false)),
      _ => None,
    }
  }

  fn is_backspace(key: &Key) -> bool {
    *key == Key::Backspace
  }

  fn is_modifier(key: &Key) -> bool {
    match key {
      Key::ControlLeft | Key::ControlRight => true,
      Key::ShiftLeft | Key::ShiftRight => true,
      Key::Alt | Key::AltGr => true,
      _ => false,
    }
  }

  fn is_stack_breaker(&self, key: &Key) -> bool {
    match key {
      Key::UpArrow | Key::DownArrow => true,
      Key::LeftArrow | Key::RightArrow => true,
      Key::PageUp | Key::PageDown => true,
      Key::Home | Key::End => true,
      _ => {
        let modifiers = self.modifiers.lock().unwrap();

        *key == Key::KeyA && modifiers.is_any_pressed(KeyboardModifiers::CONTROL_ANY)
      },
    }
  }

  fn create_print_command(key: &Key, modifiers: KeyboardModifiers) -> PrintCharCommand {
    let key = *key;

    PrintCharCommand::new(key, modifiers)
  }
}
