use std::sync::{Arc, Mutex};

use rust_logger::*;

use crate::Platform;
use crate::Settings;

use crate::components::state::State;
use crate::components::event_hub::EventHub;
use crate::components::transformers::Transformer;

use crate::common::enums::ExecutorType;
use crate::common::events::CommandEvent;

pub struct Executor {
  platform: &'static Platform,
  settings: &'static Settings,

  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,

  transformers: Vec<Box<dyn Transformer>>,
}

impl Executor {
  pub fn new(
    platform: &'static Platform,
    settings: &'static Settings,
    state: Arc<Mutex<State>>,
    event_hub: Arc<EventHub>,
    transformers: Vec<Box<dyn Transformer>>,
  ) -> Arc<Self> {
    Arc::new(Executor { platform, settings, state, event_hub, transformers })
  }

  pub fn init(self: &Arc<Self>) {
    log!("<$>Executor</>: Init");

    self.subscribe();
  }

  fn subscribe(self: &Arc<Self>) {
    let mut command_rx = self.event_hub.command_stream();

    let this = Arc::clone(self);

    tokio::task::spawn_local(async move {
      while let Ok(event) = command_rx.recv().await {
        this.process_event(event);
      }
    });
  }

  fn process_event(self: &Arc<Self>, event: CommandEvent) {
    let Some(tfr) = self.get_transformer(&event.executor.r#type) else {
      warn!("<$>Executor</>: Transformer not found: type={}", event.executor.r#type);
      return;
    };

    log!("<$>Executor</>: Apply transformer: <i&>{}</>", tfr.get_type());
  }

  fn get_transformer(&self, r#type: &ExecutorType) -> Option<&Box<dyn Transformer>> {
    self.transformers.iter().find(|tfr| tfr.get_type() == r#type)
  }
}
