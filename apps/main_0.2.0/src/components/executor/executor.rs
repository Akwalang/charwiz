use std::sync::{Arc, Mutex};

use rust_logger::*;

use crate::Platform;
use crate::Settings;

use crate::components::state::State;
use crate::components::event_hub::EventHub;
use crate::components::transformers::Transformer;

use crate::common::events::CommandEvent;

pub struct Executor {
  platform: &'static Platform,
  settings: &'static Settings,

  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,

  transformers: Vec<Box<dyn Transformer + Send + Sync>>,
}

impl Executor {
  pub fn new(
    platform: &'static Platform,
    settings: &'static Settings,
    state: Arc<Mutex<State>>,
    event_hub: Arc<EventHub>,
    transformers: Vec<Box<dyn Transformer + Send + Sync>>,
  ) -> Arc<Self> {
    Arc::new(Executor {
      platform,
      settings,
      state: state,
      event_hub: event_hub,
      transformers,
    })
  }

  pub fn init(self: &Arc<Self>) {
    log!("<$>Executor</>: Init");

    self.subscribe();
  }

  fn subscribe(self: &Arc<Self>) {
    let mut command_rx = self.event_hub.command_stream();

    let this = Arc::clone(self);

    async_std::task::spawn(async move {
      while let Ok(event) = command_rx.recv().await {
        this.process_event(event);
      }
    });
  }

  fn process_event(self: &Arc<Self>, event: CommandEvent) {
    for tfr in self.transformers.iter() {
      if *tfr.get_type() != event.executor.r#type {
        println!("Executor type: {:?}", tfr.get_type());
        continue;
      }
    }
  }
}
