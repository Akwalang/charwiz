use std::sync::{Arc, Mutex};

use rust_logger::*;

use super::emulator::Emulator;
use super::extractor::Extractor;
use super::injector::Injector;
use super::transformer::Transformer;

use crate::Platform;
use crate::Settings;

use crate::platform::common::structs::KeyboardLayoutItem;

use crate::components::state::{State, ApplicationStatus};
use crate::components::event_hub::EventHub;

use crate::common::enums::{KeyboardStateCleanupEnum, KeyboardLayoutEnum};
use crate::common::events::CommandEvent;

pub struct Executor {
  platform: &'static Platform,

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
    let extractor = Extractor::new(platform);
    let transformer = Transformer::new(platform, settings);
    let injector = Injector::new(platform, settings);

    Arc::new(Executor {
      platform,
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
        let _ = this.process_event(event).await;
      }
    });
  }

  async fn process_event(self: &Arc<Self>, event: CommandEvent) -> anyhow::Result<()> {
    self.lock_application();

    // println!("char_stack: {:?}", self.state.lock().unwrap().keyboard.char_stack);
    // println!("event_stack: {:#?}", self.state.lock().unwrap().keyboard.event_stack);

    let current_layout = self.platform.keyboard_layouts.borrow().get_current_keyboard_layout()?.clone();

    let _ = self.switch_keyboard_layout(&current_layout, &event.injector.layout_before);

    let target  = self.extractor.extract(&self.emulator, &event).await;

    let Ok(target) = target else {
      warn!("<$>Executor</>: Extraction failed: {}", target.err().unwrap());
      self.unlock_application();
      return Err(anyhow::anyhow!("Extraction failed"));
    };

    let input = self.transformer.transform(&event, &target).await;

    if let Err(error) = self.injector.inject(&self.emulator, &event, input).await {
      warn!("<$>Executor</>: Injection failed: {}", error);
    }

    if event.injector.keyboard_state_cleanup == KeyboardStateCleanupEnum::Drop {
      self.state.lock().unwrap().keyboard.stack_clear();
    }

    let _ = self.switch_keyboard_layout(&current_layout, &event.injector.layout_after);
    
    self.unlock_application();

    Ok(())
  }

  fn lock_application(&self) {
    self.state.lock().unwrap().application.set_status(ApplicationStatus::Executing);
  }

  fn unlock_application(&self) {
    self.state.lock().unwrap().application.set_status(ApplicationStatus::Active);
  }

  fn switch_keyboard_layout(&self, current: &KeyboardLayoutItem, next: &KeyboardLayoutEnum) -> anyhow::Result<()> {
    let keyboard_layouts = self.platform.keyboard_layouts.borrow();

    match next {
      KeyboardLayoutEnum::Previous => keyboard_layouts.set_previous_to_keyboard_layout(current.id.as_str())?,
      KeyboardLayoutEnum::Current => keyboard_layouts.set_keyboard_layout(current.id.as_str())?,
      KeyboardLayoutEnum::Next => keyboard_layouts.set_next_to_keyboard_layout(current.id.as_str())?,
      KeyboardLayoutEnum::Direct(layout) => {
        let Some(layout_item) = keyboard_layouts.get_keyboard_layout_by_name(&layout) else {
          return Ok(());
        };

        keyboard_layouts.set_keyboard_layout(layout_item.id.as_str())?
      },
    };

    Ok(())
  }
}
