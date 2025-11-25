use rdev::{Event, EventType, Key};

use crate::platform::Platform;
use crate::settings::Settings;

use crate::common::structs::{KeyboardEventSnapshot, KeyboardModifiers};

pub struct KeyboardState {
  platform: &'static Platform,
  settings: &'static Settings,

  pub key: Option<Key>,
  pub modifiers: KeyboardModifiers,

  pub char_stack: Vec<char>,
  pub event_stack: Vec<KeyboardEventSnapshot>,
}

impl KeyboardState {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self {
      platform,
      settings,

      key: None,
      modifiers: KeyboardModifiers::new(),

      char_stack: Vec::with_capacity(20),
      event_stack: Vec::with_capacity(20),
    }
  }

  pub fn get_string(&self) -> String {
    self.char_stack.iter().collect::<String>()
  }

  pub fn get_events(&self) -> &Vec<KeyboardEventSnapshot> {
    &self.event_stack
  }

  pub fn apply_key_event(&mut self, event: &Event) {
    match event.event_type {
      EventType::ButtonPress(_) | EventType::ButtonRelease(_) => self.process_mouse_event(event),
      EventType::KeyPress(_) | EventType::KeyRelease(_) => self.process_keyboard_event(event),
      _ => {},
    }
  }

  fn process_mouse_event(&mut self, event: &Event) {
    let EventType::ButtonPress(_) = event.event_type else { return; };

    self.stack_clear();
  }

  fn process_keyboard_event(&mut self, event: &Event) {
    let Some((key, state)) = Self::get_key(&event) else { return; };

    if KeyboardModifiers::is_modifier(&key) {
      self.handle_modifier_update(key, state);
    }

    if !KeyboardModifiers::is_modifier(&key) {
      self.key = if state { Some(key) } else { None };
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

  fn handle_modifier_update(&mut self, key: Key, state: bool) {
    if state {
      self.modifiers.add_key(&key);
    } else {
      self.modifiers.remove_key(&key);
    }
  }

  fn handle_backspace(&mut self, _: Key) {
    self.stack_pop();
  }

  fn handle_stack_breaker(&mut self, _: Key) {
    self.stack_clear();
  }

  fn handle_insert(&mut self, key: Key) {
    let settings_kl = self.settings.get_keyboard_layouts();
    let platform_kl = self.platform.get_keyboard_layouts();

    let cur_layout = platform_kl.get_current_keyboard_layout();

    let (char, is_exists) = settings_kl.find_combination(&cur_layout.name, &key, self.modifiers);

    if !is_exists { return; }

    if let Some(char) = char {
      if false
        || self.char_stack.last() == Some(&' ')
        || self.char_stack.last() == Some(&'\n')
        || self.char_stack.last() == Some(&'\t')
      {
        self.stack_clear();
      }

      self.char_stack.push(char);
    }

    self.event_stack.push(KeyboardEventSnapshot::new(Some(key), self.modifiers));
  }

  fn stack_pop(&mut self) {
    self.char_stack.pop();
    self.event_stack.pop();
  }

  fn stack_clear(&mut self) {
    self.char_stack.clear();
    self.event_stack.clear();
  }

  fn get_key(event: &Event) -> Option<(Key, bool)> {
    match event.event_type {
      EventType::KeyPress(key) => Some((key, true)),
      EventType::KeyRelease(key) => Some((key, false)),
      _ => None,
    }
  }

  fn is_backspace(key: &Key) -> bool {
    *key == Key::Backspace
  }

  fn is_stack_breaker(&self, key: &Key) -> bool {
    match key {
      Key::UpArrow | Key::DownArrow => true,
      Key::LeftArrow | Key::RightArrow => true,
      Key::PageUp | Key::PageDown => true,
      Key::Home | Key::End => true,
      _ => false
        || (*key == Key::KeyA && self.modifiers.is_any_pressed(KeyboardModifiers::CONTROL_ANY))
        || (*key == Key::KeyC && self.modifiers.is_any_pressed(KeyboardModifiers::CONTROL_ANY))
      ,
    }
  }
}
