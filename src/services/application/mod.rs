use crate::services::{Platform, PlatformTrait, Keyboard, Transformer};

pub struct Application {
  platform: Platform,
  keyboard: Keyboard,
  transformer: Transformer,
}

impl Application {
  pub fn new() -> Self {
    let platform = Platform::new();
    let keyboard = Keyboard::new();
    let transformer = Transformer::new();

    Self { platform, keyboard, transformer }
  }

  pub async fn run(&mut self) {
    self.keyboard.listen();

    println!("Application is running...");

    loop {
      let keys = self.keyboard.get_input().await;

      println!("Input: {:?}", keys);

      // let status = self.platform.get_status();
      // let input = self.platform.get_input();

      let output = self.transformer.convert(keys);

      if output.is_none() { continue; }

      // Waiting for release of keys
      loop {
        let keys = self.keyboard.get_input().await;

        if keys.is_empty() { break; }
      }

      if let Some(output) = output {
        println!("Output: {:?}", output);
        self.platform.apply_output(output);
      }
    }
  }
}
