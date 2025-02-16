use std::collections::HashSet;

use rdev::Key;

use async_std::task;
use futures::future::FutureExt;
use futures::select;

use crate::services::executor::Executor;
use crate::services::keyboard::Keyboard;

use crate::services::tray::Tray;
use crate::services::tray::enums::TrayAction;

use crate::services::config::Config;

pub struct Application;

impl Application {
  pub async fn run() {
    let mut keyboard = Keyboard::new();
    let mut tray = Tray::new();

    println!("Application is running...");

    task::spawn(async move {
      loop {
        let action = tray.get_input().await;
        Self::on_tray_input(&mut tray, action).await;
      }
    });

    task::spawn(async move {
      loop {
        let action = keyboard.get_input().await;
        Self::on_keyboard_input(&mut keyboard, action).await;
      }
    });

    Self::keep_alive().await;
  }

  async fn on_tray_input(tray: &mut Tray, action: TrayAction) {
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

  async fn on_keyboard_input(keyboard: &mut Keyboard, keys: HashSet<Key>) {
    let action = Executor::find_action(keys);

    match action {
      Some(action) => {
        keyboard.block_until_empty_input().await;

        println!("=> Lock keyboard...");

        keyboard.lock();

        if let Err(_) = Executor::execute_action(&action).await {
          println!("Failed to apply command");
        }

        println!("=> Unlock keyboard...");

        keyboard.unlock();
      },
      None => {},
    };
  }

  async fn keep_alive() {
    loop {
      task::sleep(std::time::Duration::from_secs(1)).await;
    }
  }
}
