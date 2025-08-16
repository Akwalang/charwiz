use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use rust_logger::*;

use rdev::Key;

use async_std::task;

use crate::services::executor::Executor;
use crate::services::keyboard::Keyboard;
use crate::services::platform::{Platform, PlatformTrait};

use crate::services::tray::Tray;
use crate::services::tray::enums::TrayAction;

use crate::services::config::Config;

pub struct Application;

impl Application {
  pub async fn run() {
    let keyboard = Arc::new(RwLock::new(Keyboard::new()));
    let executor = Executor::new();

    log!("Application is running...");

    let keyboard_tray_clone = Arc::clone(&keyboard);

    task::spawn(async move {
      let tray = Tray::new();

      loop {
        let action = tray.get_input().await;
        Self::on_tray_input(&tray, &keyboard_tray_clone, action).await;
      }
    });

    let keyboard_kbd_clone = Arc::clone(&keyboard);

    loop {
      let kbd = keyboard_kbd_clone.read().unwrap();
      let action = kbd.get_input().await;
      drop(kbd);

      Self::on_keyboard_input(&keyboard_kbd_clone, &executor, action).await;
    }
  }

  async fn on_tray_input(_tray: &Tray, keyboard: &Arc<RwLock<Keyboard>>, action: TrayAction) {
    match action {
      TrayAction::Reload => {
        log!("Reloading settings...");
        Config::get_instance().reload_settings();
        log!("<green>Reloading completed</>");
      },
      TrayAction::Diagnostic => {
        log!("Copying diagnostic info to clipboard...");
        Executor::copy_diagnostic(Self::get_diagnostic(&keyboard.read().unwrap()));
        log!("Diagnostic data <green>copied to clipboard</>");
      },
      TrayAction::Exit => {
        log!("Application is closing...");
        std::process::exit(0);
      }
    }
  }

  async fn on_keyboard_input(keyboard: &Arc<RwLock<Keyboard>>, executor: &Executor, keys: HashSet<Key>) {
    let action = Executor::find_action(keys);

    match action {
      Some(action) => {
        let mut keyboard = keyboard.write().unwrap();

        keyboard.block_until_empty_input().await;

        debug!("=> Lock keyboard...");

        keyboard.lock();

        if let Err(_) = executor.execute_action(&action).await {
          error!("Failed to apply command");
        }

        debug!("=> Unlock keyboard...");

        keyboard.unlock();
      },
      None => {},
    };
  }

  fn get_diagnostic(keyboard: &Keyboard) -> String {
    let platform = Platform::get_instance();

    let mut info = String::new();

    info.push_str("Diagnostic info:\n\n");
    info.push_str(&platform.get_diagnostic());
    info.push_str("\n");
    info.push_str(&keyboard.get_diagnostic());

    info
  }
}
