use std::sync::Arc;

use rust_logger::*;

use crate::components::event_hub::EventHub;
use crate::components::state::State;

pub struct Executor {
  state: Arc<State>,
  event_hub: Arc<EventHub>,
}

impl Executor {
  pub fn new(state: &Arc<State>, event_hub: &Arc<EventHub>) -> Self {
    Executor {
      state: state.clone(),
      event_hub: event_hub.clone(),
    }
  }

  pub fn init(&self) {
    log!("<$>Executor</>: Init");

    self.subscribe();
  }

  fn subscribe(&self) {
    let mut command_rx = self.event_hub.command_stream();

    async_std::task::spawn(async move {
      while let Ok(command) = command_rx.recv().await {
        log!("<$>Executor</>: Received command event: {:?}", command);
      }
    });
  }
}
