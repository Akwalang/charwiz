use std::collections::HashSet;

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
    let mut keyboard = Keyboard::new();
    let executor = Executor::new();

    println!("Application is running...");

    task::spawn(async {
      let tray = Tray::new();

      loop {
        let action = tray.get_input().await;
        Self::on_tray_input(&tray, action).await;
      }
    });

    loop {
      let action = keyboard.get_input().await;
      Self::on_keyboard_input(&mut keyboard, &executor, action).await;
    }
  }

  async fn on_tray_input(_tray: &Tray, action: TrayAction) {
    match action {
      TrayAction::Reload => {
        println!("Reloading settings...");
        Config::get_instance().reload_settings();
        println!("Reloading completed");
      },
      TrayAction::Exit => {
        println!("Application is closing...");
        std::process::exit(0);
      }
    }
  }

  async fn on_keyboard_input(keyboard: &mut Keyboard, executor: &Executor, keys: HashSet<Key>) {
    let action = Executor::find_action(keys);

    match action {
      Some(action) => {
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
