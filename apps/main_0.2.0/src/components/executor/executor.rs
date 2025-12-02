use std::sync::{Arc, Mutex};

use rust_logger::*;

use super::emulator::Emulator;
use super::extractor::Extractor;
use super::injector::Injector;
use super::transformer::Transformer;

use crate::Platform;
use crate::Settings;

use crate::components::state::{State, ApplicationStatus};
use crate::components::event_hub::EventHub;

use crate::common::enums::{TransformTargetEnum, ExecutorTypeEnum};
use crate::common::events::CommandEvent;

pub struct Executor {
  platform: &'static Platform,
  settings: &'static Settings,

  emulator: Emulator,
  extractor: Extractor,
  transformer: Transformer,
  injector: Injector,

  state: Arc<Mutex<State>>,
  event_hub: Arc<EventHub>,
}

impl Executor {
  pub fn new(
    platform: &'static Platform,
    settings: &'static Settings,
    state: Arc<Mutex<State>>,
    event_hub: Arc<EventHub>,
  ) -> Arc<Self> {
    let emulator = Emulator::new(settings);
    let extractor = Extractor::new(platform, state.clone());
    let transformer = Transformer::new(platform, settings);
    let injector = Injector::new(platform, settings);

    Arc::new(Executor {
      platform, settings,
      emulator, extractor, transformer, injector,
      state, event_hub,
    })
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
    self.lock_application();

    let target  = self.extractor.extract(&self.emulator, &event).await;

    let Ok(target) = target else {
      warn!("<$>Executor</>: Extraction failed: {}", target.err().unwrap());
      self.unlock_application();
      return;
    };

    println!("Extracted target: {:?}", target);

    let value = self.transformer.transform(&event, &target).await;

    if let Err(error) = self.injector.inject(&self.emulator, event, value).await {
      warn!("<$>Executor</>: Injection failed: {}", error);
    }

    self.unlock_application();
  }

  fn lock_application(&self) {
    self.state.lock().unwrap().application.set_status(ApplicationStatus::Executing);
  }

  fn unlock_application(&self) {
    self.state.lock().unwrap().application.set_status(ApplicationStatus::Active);
  }
}
