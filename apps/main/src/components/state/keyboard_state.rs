use rdev::{Event, EventType, Key};

#[cfg(feature = "logger")]
use logger::*;

use settings_core::structs::{KeyboardSnapshot, KeyboardModifiers};

use crate::platform::Platform;
use crate::settings::Settings;

use crate::platform::common::structs::KeyboardLayoutItem;

pub struct KeyboardState {
  platform: &'static Platform,
  settings: &'static Settings,

  keyboard_snapshot: KeyboardSnapshot,

  initial_keyboard_layout: KeyboardLayoutItem,

  char_stack: Vec<char>,
  event_stack: Vec<(bool, KeyboardSnapshot)>,
}

impl KeyboardState {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self {
      platform,
      settings,

      initial_keyboard_layout: platform.keyboard_layouts.borrow().get_first_layout(),
      keyboard_snapshot: KeyboardSnapshot::default(),

      char_stack: Vec::with_capacity(20),
      event_stack: Vec::with_capacity(20),
    }
  }

  pub fn get_initial_keyboard_layout(&self) -> &KeyboardLayoutItem {
    &self.initial_keyboard_layout
  }

  pub fn set_initial_keyboard_layout(&mut self, kbl: KeyboardLayoutItem) {
    self.initial_keyboard_layout = kbl;
  }

  pub fn get_current_snapshot(&self) -> &KeyboardSnapshot {
    &self.keyboard_snapshot
  }

  pub fn get_char_stack(&self) -> &Vec<char> {
    &self.char_stack
  }

  pub fn get_chars(&self) -> Vec<char> {
    self.char_stack.clone()
  }

  pub fn get_events(&self) -> Vec<KeyboardSnapshot> {
    self.event_stack.iter().map(|item| item.1.clone()).collect()
  }

  pub fn apply_key_event(&mut self, event: &Event, is_executing: bool) {
    match event.event_type {
      EventType::ButtonPress(_) | EventType::ButtonRelease(_) => self.process_mouse_event(event),
      EventType::KeyPress(_) | EventType::KeyRelease(_) => self.process_keyboard_event(event, is_executing),
      _ => {},
    }
  }

  fn process_mouse_event(&mut self, event: &Event) {
    let EventType::ButtonPress(_) = event.event_type else { return; };

    self.stack_clear();
  }

  fn process_keyboard_event(&mut self, event: &Event, is_executing: bool) {
    let Some((key, state)) = Self::get_key(&event) else { return; };

    if KeyboardModifiers::is_modifier(key) {
      self.handle_modifier_update(key, state);
    } else {
      self.keyboard_snapshot.key = if state { Some(key) } else { None };
    }

    if !state || is_executing { return; }

    if self.is_banned_event() {
      self.state_clear();
    }

    if self.is_stack_breaker(key) {
      self.handle_stack_breaker(key);
    } else if Self::is_backspace(key) {
      self.handle_backspace(key);
    } else {
      self.handle_insert(key, is_executing);
    }
  }

  fn handle_modifier_update(&mut self, key: Key, state: bool) {
    if state {
      self.keyboard_snapshot.modifiers.add_key(key);
    } else {
      self.keyboard_snapshot.modifiers.remove_key(key);
    }
  }

  fn handle_backspace(&mut self, _: Key) {
    self.stack_pop();
  }

  fn handle_stack_breaker(&mut self, _: Key) {
    self.stack_clear();
  }

  fn handle_insert(&mut self, key: Key, is_executing: bool) {
    let platform_kl = self.platform.keyboard_layouts.borrow();
    let settings_kl = self.settings.keyboard_layouts.borrow();

    let switchers = self.settings.get_switch_keyboard_layout_hotkeys();

    let Ok(cur_layout) = platform_kl.get_current_keyboard_layout() else {
      return;
    };

    let (char, is_char_exists) = settings_kl.find_combination(&cur_layout.name, key, self.keyboard_snapshot.modifiers);
    let is_switch_exists = switchers.iter().any(|kb| *kb == self.keyboard_snapshot);

    // skip registration when hotkey missing in all keyboard layouts
    // but save when any of layouts has this hotkey
    if !is_char_exists && !is_switch_exists { return; }

    if is_switch_exists && self.char_stack.is_empty() { return; }

    if let Some(char) = char {
      if false
        || self.char_stack.last() == Some(&' ')
        || self.char_stack.last() == Some(&'\n')
        || self.char_stack.last() == Some(&'\t')
      {
        self.stack_clear();
      }

      if !is_executing && self.char_stack.len() == 0 {
        self.set_initial_keyboard_layout(cur_layout.clone());
      }

      self.char_stack.push(char);
    }

    self.event_stack.push((char.is_some(), KeyboardSnapshot::new(Some(key), self.keyboard_snapshot.modifiers)));
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

  fn state_clear(&mut self) {
    self.keyboard_snapshot.clear();
  }

  pub fn stack_clear(&mut self) {
    #[cfg(feature = "logger")]
    debug!("<$>Keyboard State</>: Drop keyboard stack");

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
  fn is_backspace(key: Key) -> bool {
    key == Key::Backspace
  }

  fn is_banned_event(&self) -> bool {
    let hotkeys = self.settings.get_banned_hotkeys();

    hotkeys.iter().any(|ks| *ks == self.keyboard_snapshot)
  }

  fn is_stack_breaker(&self, key: Key) -> bool {
    let snapshot = KeyboardSnapshot::new(Some(key), self.keyboard_snapshot.modifiers.clone());
    let hotkeys = self.settings.get_stack_brake_hotkeys();

    hotkeys.iter().any(|ks| *ks == snapshot)
  }
}
