use std::sync::{Arc, Mutex};

use rust_logger::*;

use crate::Platform;
use crate::Settings;

use crate::components::state::State;
use crate::components::event_hub::EventHub;
use crate::components::transformers::Transformer;
use crate::components::executor::emulator::Emulator;
use crate::components::injector::Injector;

use crate::common::enums::{TransformTargetEnum, ExecutorTypeEnum};
use crate::common::events::CommandEvent;

pub struct Executor {
  platform: &'static Platform,
  settings: &'static Settings,

  emulator: Emulator,

  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,

  transformers: Vec<Box<dyn Transformer>>,
  injector: Injector,
}

impl Executor {
  pub fn new(
    platform: &'static Platform,
    settings: &'static Settings,
    state: Arc<Mutex<State>>,
    event_hub: Arc<EventHub>,
    transformers: Vec<Box<dyn Transformer>>,
    injector: Injector,
  ) -> Arc<Self> {
    let emulator = Emulator::new(settings);

    Arc::new(Executor { platform, settings, emulator, state, event_hub, transformers, injector })
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
        this.process_event(event).await;
      }
    });
  }

  async fn process_event(self: &Arc<Self>, event: CommandEvent) {
    let Some(tfr) = self.get_transformer(&event.executor.r#type) else {
      warn!("<$>Executor</>: Transformer not found: type={}", event.executor.r#type);
      return;
    };

    let target = self.get_transform_value(&event).await;

    let result = tfr.transform(&event, &target);

    self.emulator.test().await;

    // self.injector.inject(event, result).await;
  }

  fn get_transformer(&self, r#type: &ExecutorTypeEnum) -> Option<&Box<dyn Transformer>> {
    self.transformers.iter().find(|tfr| tfr.get_type() == r#type)
  }

  async fn get_transform_value(&self, event: &CommandEvent) -> String {
    let target = &event.injector.target;

    match target {
      TransformTargetEnum::None => String::from(""),
      _ => String::from(""),
    }
  }
}
