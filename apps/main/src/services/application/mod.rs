use crate::services::{
  Keyboard,
  Executor,
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

      let action = Executor::find_action(keys);

      println!("Action: {:?}", action);

      match action {
        Some(action) => {
          self.block_until_empty_input().await;

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

  async fn block_until_empty_input(&mut self) {
    loop {
      let keys = self.keyboard.get_input().await;

      if keys.is_empty() { break; }
    }
  }
}
