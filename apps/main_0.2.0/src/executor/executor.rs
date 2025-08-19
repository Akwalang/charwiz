use std::sync::Arc;

use rust_logger::*;

use crate::event_hub::EventHub;

pub struct Executor {
  event_hub: Arc<EventHub>,
}

impl Executor {
  pub fn new(event_hub: Arc<EventHub>) -> Self {
    Executor {
      event_hub,
    }
  }

  pub fn start(&self) {
    log!("<purple>Executor</>: Initialize");

    self.subscribe();
  }

  fn subscribe(&self) {
    let mut command_rx = self.event_hub.command_stream();

    async_std::task::spawn(async move {
      while let Ok(command) = command_rx.recv().await {
        log!("<purple>Executor</>: Received command event: {:?}", command);
      }
    });
  }
}
