use super::KeyEvent;

use std::collections::HashSet;

use async_std::{channel, task};
use async_std::channel::{Receiver, Sender};

use rdev::{listen, Event, EventType, Key};

pub struct Keyboard {
  active: HashSet<Key>,
  sender: Sender<KeyEvent>,
  receiver: Receiver<KeyEvent>,
}

impl Keyboard {
  pub fn new() -> Self {
    let active = HashSet::new();

    let (sender, receiver) = channel::unbounded::<KeyEvent>();

    Self { active, sender, receiver }
  }

  pub fn listen(&mut self) {
    let sender = self.sender.clone();

    task::spawn(async move {
      let callback = move |event| {
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
          KeyEvent::KeyDown(key) => { self.active.insert(key); },
          KeyEvent::KeyUp(key) => { self.active.remove(&key); },
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
}
