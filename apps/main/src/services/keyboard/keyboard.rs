use crate::services::keyboard::enums::KeyEvent;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use async_std::{task, channel};

use rdev::{listen, Event, EventType, Key};

pub struct Keyboard {
  is_locked: Arc<Mutex<bool>>,
  active: Arc<Mutex<HashSet<Key>>>,
  sender: channel::Sender<HashSet<Key>>,
  receiver: channel::Receiver<HashSet<Key>>,
}

impl Keyboard {
  pub fn new() -> Self {
    let is_locked = Arc::new(Mutex::new(false));
    let active = Arc::new(Mutex::new(HashSet::new()));

    let (sender, receiver) = channel::unbounded::<HashSet<Key>>();

    let keyboard = Self { is_locked, active, sender, receiver };

    keyboard.listen();

    keyboard
  }

  pub fn lock(&mut self) {
    *self.is_locked.lock().unwrap() = true;
  }

  pub fn unlock(&mut self) {
    *self.is_locked.lock().unwrap() = false;
  }

  pub fn listen(&self) {
    let sender = self.sender.clone();
    let locked = self.is_locked.clone();
    let active = self.active.clone();

    let callback = move |event: Event| {
      match event.event_type {
        EventType::KeyPress(key) => {
          println!("Key pressed: {:?}", key);
        },
        EventType::KeyRelease(key) => {
          println!("Key released: {:?}", key);
        },
        _ => {},
      }

      if *locked.lock().unwrap() { return; }

      if let Some(event) = Self::convert(event) {
        let mut active = active.lock().unwrap();

        match event {
          KeyEvent::KeyDown(key) => {
            if let Key::Unknown(_) = key { return; }
            active.insert(key);
          },
          KeyEvent::KeyUp(key) => {
            if let Key::Unknown(_) = key { return; }
            active.remove(&key);
          },
        }

        sender.send_blocking(active.clone()).unwrap();
      }
    };

    task::spawn(async move {
      if let Err(error) = listen(callback) {
        panic!("Can't start listen keyboard actions.\nError: {:?}", error);
      }
    });
  }

  pub async fn get_input(&self) -> HashSet<Key> {
    loop {
      if let Ok(event) = self.receiver.recv().await {
        return event;
      }
    }
  }

  fn convert(event: Event) -> Option<KeyEvent> {
    match event.event_type {
      EventType::KeyPress(key) => Some(KeyEvent::KeyDown(key)),
      EventType::KeyRelease(key) => Some(KeyEvent::KeyUp(key)),
      _ => None,
    }
  }

  pub async fn block_until_empty_input(&mut self) {
    loop {
      let keys = self.get_input().await;

      if keys.is_empty() { break; }
    }
  }
}
