use std::sync::Arc;

use rust_logger::*;

use crate::components::event_hub::EventHub;

pub struct Executor {
  event_hub: Arc<EventHub>,
}

impl Executor {
  pub fn new(event_hub: &Arc<EventHub>) -> Self {
    Executor {
      event_hub: event_hub.clone(),
    }
  }

  pub fn init(&self) {
    log!("<purple>Executor</>: Init");

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
