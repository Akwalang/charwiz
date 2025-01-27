use super::KeyEvent;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use async_std::{channel, task};
use async_std::channel::{Receiver, Sender};

use rdev::{listen, Event, EventType, Key};

pub struct Keyboard {
  is_locked: Arc<Mutex<bool>>,
  active: HashSet<Key>,
  sender: Sender<KeyEvent>,
  receiver: Receiver<KeyEvent>,
}

impl Keyboard {
  pub fn new() -> Self {
    let is_locked = Arc::new(Mutex::new(false));
    let active = HashSet::new();

    let (sender, receiver) = channel::unbounded::<KeyEvent>();

    Self { is_locked, active, sender, receiver }
  }

  pub fn lock(&mut self) {
    *self.is_locked.lock().unwrap() = true;
  }

  pub fn unlock(&mut self) {
    *self.is_locked.lock().unwrap() = false;
  }

  pub fn listen(&mut self) {
    let sender = self.sender.clone();

    let arc_clone = self.is_locked.clone();

    task::spawn(async move {
      let callback = move |event| {
        if *arc_clone.lock().unwrap() { return; }

        if let Some(event) = Self::convert(event) {
          sender.send_blocking(event).unwrap();
        }
      };

      if let Err(error) = listen(callback) {
        panic!("Can't start listen keyboard actions.\nError: {:?}", error);
      }
    });
  }

  pub async fn get_input(&mut self) -> HashSet<Key> {
    loop {
      if let Ok(event) = self.receiver.recv().await {
        match event {
          KeyEvent::KeyDown(key) => {
            if let Key::Unknown(_) = key { continue; }
            self.active.insert(key);
          },
          KeyEvent::KeyUp(key) => {
            if let Key::Unknown(_) = key { continue; }
            self.active.remove(&key);
          },
        }

        return self.active.clone();
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
