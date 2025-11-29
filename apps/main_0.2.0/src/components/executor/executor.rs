use std::sync::{Arc, Mutex};

use rust_logger::*;

use super::emulator::Emulator;
use super::extractor::Extractor;
use super::injector::Injector;

use crate::Platform;
use crate::Settings;

use crate::components::state::State;
use crate::components::event_hub::EventHub;
use crate::components::transformers::Transformer;

use crate::common::enums::{TransformTargetEnum, ExecutorTypeEnum};
use crate::common::events::CommandEvent;

pub struct Executor {
  platform: &'static Platform,
  settings: &'static Settings,

  emulator: Emulator,
  extractor: Extractor,
  injector: Injector,

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
    let emulator = Emulator::new(settings);

    let extractor = Extractor::new(platform, state.clone());
    let injector = Injector::new(platform, settings);

    Arc::new(Executor { platform, settings, emulator, extractor, injector, state, event_hub, transformers })
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

    // lock state

    let target  = self.extractor.extract(&self.emulator, &event).await;

    let Ok(target) = target else {
      warn!("<$>Executor</>: Extraction failed: {}", target.err().unwrap());
      return;
    };

    println!("Extracted target: {:?}", target);

    // let value = self.get_transform_value(&event).await;

    // let result = tfr.transform(&event, &value);

    // let _= self.injector.inject(&self.emulator, event, result).await;

    // unlock state
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
