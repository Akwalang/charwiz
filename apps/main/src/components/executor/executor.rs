use std::cell::RefCell;
use std::rc::Rc;

#[cfg(feature = "logger")]
use logger::*;

use tokio::sync::broadcast::error::RecvError;

use settings_core::enums::{KeyboardStateCleanupEnum, KeyboardLayoutEnum};

use super::emulator::Emulator;
use super::extractor::Extractor;
use super::injector::Injector;
use super::transformer::Transformer;

use crate::Platform;
use crate::Settings;

use crate::platform::common::structs::KeyboardLayoutItem;

use crate::components::state::{State, ApplicationStatus};
use crate::components::event_hub::EventHub;

use crate::common::events::CommandEvent;

pub struct Executor {
  platform: &'static Platform,

  emulator: Emulator,
  extractor: Extractor,
  transformer: Transformer,
  injector: Injector,

  state: Rc<RefCell<State>>,
  event_hub: Rc<EventHub>,
}

impl Executor {
  pub fn new(
    platform: &'static Platform,
    settings: &'static Settings,
    state: Rc<RefCell<State>>,
    event_hub: Rc<EventHub>,
  ) -> Rc<Self> {
    let emulator = Emulator::new(settings);
    let extractor = Extractor::new(platform, settings);
    let transformer = Transformer::new(platform, settings);
    let injector = Injector::new(platform, settings);

    Rc::new(Executor {
      platform,
      emulator, extractor, transformer, injector,
      state, event_hub,
    })
  }

  pub fn init(self: &Rc<Self>) {
    #[cfg(feature = "logger")]
    log!("<$>Executor</>: Init");

    self.subscribe();
  }

  fn subscribe(self: &Rc<Self>) {
    let mut command_rx = self.event_hub.command_stream();

    let this = Rc::clone(self);

    tokio::task::spawn_local(async move {
      loop {
        match command_rx.recv().await {
          Ok(event) => {
            let _ = this.process_event(event).await;
          },
          Err(RecvError::Lagged(_)) => continue,
          Err(RecvError::Closed) => break,
        }
      }
    });
  }

  async fn process_event(self: &Rc<Self>, event: CommandEvent) -> anyhow::Result<()> {
    self.lock_application();

    let current_layout = self.platform.keyboard_layouts.borrow().get_current_keyboard_layout()?.clone();

    let _ = self.switch_keyboard_layout(&current_layout, &event.injector.layout_before);

    let target  = self.extractor.extract(&self.emulator, &event).await;

    let Ok(target) = target else {
      #[cfg(feature = "logger")]
      warn!("<$>Executor</>: Extraction failed: {}", target.err().unwrap());
      self.unlock_application();
      anyhow::bail!("Extraction failed");
    };

    let input = self.transformer.transform(&event, &target).await;

    let _inject_result  = self.injector.inject(&self.emulator, &event, input).await;

    #[cfg(feature = "logger")]
    if let Err(error)  = _inject_result {
      warn!("<$>Executor</>: Injection failed: {}", error);
    }

    if event.injector.keyboard_state_cleanup == KeyboardStateCleanupEnum::Drop {
      self.state.borrow_mut().keyboard.stack_clear();
    }

    let _ = self.switch_keyboard_layout(&current_layout, &event.injector.layout_after);
    
    self.unlock_application();

    Ok(())
  }

  fn lock_application(&self) {
    self.state.borrow_mut().application.set_status(ApplicationStatus::Executing);
  }

  fn unlock_application(&self) {
    self.state.borrow_mut().application.set_status(ApplicationStatus::Listening);
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
