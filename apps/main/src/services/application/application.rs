use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use rdev::Key;

use async_std::task;

use crate::services::executor::Executor;
use crate::services::keyboard::Keyboard;

use crate::services::tray::Tray;
use crate::services::tray::enums::TrayAction;

use crate::services::config::Config;

pub struct Application;

impl Application {
  pub async fn run() {
    let keyboard = Arc::new(RwLock::new(Keyboard::new()));
    let executor = Executor::new();

    println!("Application is running...");

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
        println!("Reloading settings...");
        Config::get_instance().reload_settings();
        println!("Reloading completed");
      },
      TrayAction::Diagnostic => {
        println!("Coping diagnostic info to clipboard...");
        Executor::copy_diagnostic(keyboard.read().unwrap().get_diagnostic());
      },
      TrayAction::Exit => {
        println!("Application is closing...");
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

        println!("=> Lock keyboard...");

        keyboard.lock();

        if let Err(_) = executor.execute_action(&action).await {
          println!("Failed to apply command");
        }

        println!("=> Unlock keyboard...");

        keyboard.unlock();
      },
      None => {},
    };
  }
}
