use crate::services::{
  Platform,
  Keyboard,
  Executor,
  Hotkeys,
};

pub struct Application {
  platform: Platform,
  keyboard: Keyboard,
  executor: Executor,
}

impl Application {
  pub fn new() -> Self {
    let platform = Platform::new();
    let keyboard = Keyboard::new();
    let executor = Executor::new();

    Self { platform, keyboard, executor }
  }

  pub async fn run(&mut self) {
    self.keyboard.listen();

    println!("Application is running...");

    loop {
      let keys = self.keyboard.get_input().await;

      println!("Input: {:?}", keys);

      // let status = self.platform.get_status();
      // let input = self.platform.get_input();

      let command = Hotkeys::convert(keys);

      if command.is_none() { continue; }

      // Waiting for release of keys
      loop {
        let keys = self.keyboard.get_input().await;

        if keys.is_empty() { break; }
      }

      self.executor.apply(command.unwrap()).await;
    }
  }
}
