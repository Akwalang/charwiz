use crate::services::{
  Keyboard,
  Executor,
};

use super::Tray;

pub struct Application {
  keyboard: Keyboard,
  executor: Executor,
  tray: Tray,
}

impl Application {
  pub fn new() -> Self {
    let keyboard = Keyboard::new();
    let executor = Executor::new();
    let tray = Tray::new();

    Self { keyboard, executor, tray }
  }

  pub async fn run(&mut self) {
    self.keyboard.listen();

    println!("Application is running...");

    loop {
      let keys = self.keyboard.get_input().await;

      let action = Executor::find_action(keys);

      match action {
        Some(action) => {
          self.keyboard.block_until_empty_input().await;

          self.keyboard.lock();

          if let Err(_) = self.executor.apply(action).await {
            println!("Failed to apply command");
          }

          self.keyboard.unlock();
        },
        None => { continue; },
      };
    }
  }
}
