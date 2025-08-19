use std::sync::Arc;

use rust_logger::*;

use crate::event_hub::{EventHub, InputEvent};

pub struct CommandDetector {
  event_hub: Arc<EventHub>,
  stack: Vec<InputEvent>,
}

impl CommandDetector {
  pub fn new(event_hub: Arc<EventHub>) -> Arc<Self> {
    Arc::new(CommandDetector { event_hub, stack: Vec::new() })
  }

  pub fn start(self: &Arc<Self>) {
    log!("<purple>CommandDetector</>: Initialize");

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
    // log!("<purple>CommandDetector</>: Processing event: {:?}", event);
  }
}
