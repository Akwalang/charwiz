use rdev::{Event, EventType, Key};

use rust_logger::*;

use crate::platform::Platform;
use crate::settings::Settings;

use crate::common::structs::{KeyboardEventSnapshot, KeyboardModifiers};

pub struct KeyboardState {
  platform: &'static Platform,
  settings: &'static Settings,

  pub key: Option<Key>,
  pub modifiers: KeyboardModifiers,

  pub initial_keyboard_layout: String,

  pub char_stack: Vec<char>,
  pub event_stack: Vec<(bool, KeyboardEventSnapshot)>,
}

impl KeyboardState {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self {
      platform,
      settings,

      key: None,
      modifiers: KeyboardModifiers::default(),

      initial_keyboard_layout: String::new(),

      char_stack: Vec::with_capacity(20),
      event_stack: Vec::with_capacity(20),
    }
  }

  pub fn get_string(&self) -> String {
    self.char_stack.iter().collect::<String>()
  }

  pub fn get_chars(&self) -> Vec<char> {
    self.char_stack.clone()
  }

  pub fn get_events(&self) -> Vec<KeyboardEventSnapshot> {
    self.event_stack.iter().map(|item| item.1.clone()).collect()
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

    let platform_kl = self.platform.keyboard_layouts.borrow();

    // if platform_kl.is_banned_event_snapshots() {

    // }

    if self.is_stack_breaker(&key) {
      self.handle_stack_breaker(key);
    } else if Self::is_backspace(&key) {
      self.handle_backspace(key);
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
    let platform_kl = self.platform.keyboard_layouts.borrow();
    let settings_kl = self.settings.keyboard_layouts.borrow();

    let Ok(cur_layout) = platform_kl.get_current_keyboard_layout() else {
      return;
    };

    let cur_layout_name = cur_layout.name.to_owned();

    let (char, is_exists) = settings_kl.find_combination(&cur_layout_name, &key, self.modifiers);

    // skip registration when hotkey missing in all keyboard layouts
    // but save when any of layouts has this hotkey
    if !is_exists { return; }

    if let Some(char) = char {
      if false
        || self.char_stack.last() == Some(&' ')
        || self.char_stack.last() == Some(&'\n')
        || self.char_stack.last() == Some(&'\t')
      {
        self.stack_clear();
      }

      if self.char_stack.len() == 0 {
        self.initial_keyboard_layout = cur_layout_name;
      }

      self.char_stack.push(char);
    }

    self.event_stack.push((char.is_some(), KeyboardEventSnapshot::new(Some(key), self.modifiers)));
  }

  fn stack_pop(&mut self) {
    self.char_stack.pop();

    loop {
      let Some((state, _)) = self.event_stack.pop() else {
        break;
      };

      if state { break; }
    }
  }

  pub fn stack_clear(&mut self) {
    debug!("<$>KeyboardState</>: Drop keyboard stack");

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

  #[inline]
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
        || (*key == Key::Tab && self.modifiers == KeyboardModifiers::new(KeyboardModifiers::ALT_ANY))
        || (*key == Key::Backspace && self.modifiers == KeyboardModifiers::new(KeyboardModifiers::CONTROL_ANY))
        || (*key == Key::KeyA && self.modifiers == KeyboardModifiers::new(KeyboardModifiers::CONTROL_ANY))
        || (*key == Key::KeyX && self.modifiers == KeyboardModifiers::new(KeyboardModifiers::CONTROL_ANY))
        || (*key == Key::KeyC && self.modifiers == KeyboardModifiers::new(KeyboardModifiers::CONTROL_ANY))
        || (*key == Key::KeyY && self.modifiers == KeyboardModifiers::new(KeyboardModifiers::CONTROL_ANY))
        || (*key == Key::KeyZ && self.modifiers == KeyboardModifiers::new(KeyboardModifiers::CONTROL_ANY))
        || (
            *key == Key::KeyZ
            && self.modifiers == KeyboardModifiers::new(KeyboardModifiers::CONTROL_ANY)
            && self.modifiers.is_any_pressed(KeyboardModifiers::SHIFT_ANY)
          )
      ,
    }
  }
}
