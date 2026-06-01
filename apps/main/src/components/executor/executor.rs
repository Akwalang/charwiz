use std::cell::RefCell;
use std::rc::Rc;

#[cfg(feature = "logger")]
use logger::*;

use tokio::time::{sleep, Duration};
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

use crate::utils::spin_lock::spin_sleep;

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
    let _app_lock_guard = self.lock_application();

    spin_sleep(std::time::Duration::from_millis(5));

    let current_layout = self.state.borrow().keyboard.get_initial_keyboard_layout().clone();
    let kbl = self.switch_keyboard_layout(&current_layout, &event.injector.layout_before);

    if let Ok(Some(kbl)) = kbl {
      self.state.borrow_mut().keyboard.set_initial_keyboard_layout(kbl);
    };

    let target  = self.extractor.extract_target(&self.emulator, &event).await;

    let Ok(target) = target else {
      #[cfg(feature = "logger")]
      warn!("<$>Executor</>: Target extraction failed: {}", target.err().unwrap());
      anyhow::bail!("Target extraction failed");
    };

    let context = self.extractor.extract_context(&self.emulator, &event).await;

    let Ok(context) = context else {
      #[cfg(feature = "logger")]
      warn!("<$>Executor</>: Context extraction failed: {}", context.err().unwrap());
      anyhow::bail!("Context extraction failed");
    };

    let input = self.transformer.transform(&event, &target, &context).await;

    let _inject_result  = self.injector.inject(&self.emulator, &event, input).await;

    #[cfg(feature = "logger")]
    if let Err(error)  = _inject_result {
      warn!("<$>Executor</>: Injection failed: {}", error);
    }

    // Must sleep to shade events_hub events before unlock the application
    // time can evaluate to 15 ms based on os timers but it's fine
    sleep(Duration::from_millis(1)).await;

    if event.injector.keyboard_state_cleanup == KeyboardStateCleanupEnum::Drop {
      self.state.borrow_mut().keyboard.stack_clear();
    }

    let current_layout = self.platform.keyboard_layouts.borrow().get_current_keyboard_layout()?.clone();
    let _ = self.switch_keyboard_layout(&current_layout, &event.injector.layout_after);

    Ok(())
  }

  fn lock_application(&self) -> ApplicationLockGuard {
    ApplicationLockGuard::new(self.state.clone())
  }

  fn switch_keyboard_layout(&self, current: &KeyboardLayoutItem, next: &KeyboardLayoutEnum) -> anyhow::Result<Option<KeyboardLayoutItem>> {
    let keyboard_layouts = self.platform.keyboard_layouts.borrow();

    match next {
      KeyboardLayoutEnum::Previous => keyboard_layouts.set_previous_to_keyboard_layout(current.id.as_str()),
      KeyboardLayoutEnum::Current => keyboard_layouts.set_keyboard_layout(current.id.as_str()),
      KeyboardLayoutEnum::Next => keyboard_layouts.set_next_to_keyboard_layout(current.id.as_str()),
      KeyboardLayoutEnum::Direct(layout) => {
        let Some(layout_item) = keyboard_layouts.get_keyboard_layout_by_name(&layout) else {
          return Ok(None);
        };

        keyboard_layouts.set_keyboard_layout(layout_item.id.as_str())
      },
    }
  }
}

struct ApplicationLockGuard {
  state: Rc<RefCell<State>>,
}

impl ApplicationLockGuard {
  pub fn new(state: Rc<RefCell<State>>) -> Self {
    state.borrow_mut().application.set_status(ApplicationStatus::Executing);

    Self { state }
  }
}

impl Drop for ApplicationLockGuard {
  fn drop(&mut self) {
    self.state.borrow_mut().application.set_status(ApplicationStatus::Listening);
  }
}
