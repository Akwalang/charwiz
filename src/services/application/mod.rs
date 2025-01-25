use crate::services::{
  Keyboard,
  Executor,
  Hotkeys,
};

pub struct Application {
  keyboard: Keyboard,
  executor: Executor,
}

impl Application {
  pub fn new() -> Self {
    let keyboard = Keyboard::new();
    let executor = Executor::new();

    Self { keyboard, executor }
  }

  pub async fn run(&mut self) {
    self.keyboard.listen();

    println!("Application is running...");

    loop {
      let keys = self.keyboard.get_input().await;

      // println!("Input: {:?}", keys);

      let command = Hotkeys::convert(keys);

      match command {
        Some(command) => {
          loop {
            let keys = self.keyboard.get_input().await;

            if keys.is_empty() { break; }
          }

          if let Err(_) = self.executor.apply(command).await {
            println!("Failed to apply command");
          }
        },
        None => { continue; },
      };
    }
  }
}
