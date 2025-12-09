use rust_logger::*;

use crate::Platform;
use crate::Settings;

use super::super::traits::Transformer;

use crate::components::executor::enums::InputType;

use crate::common::events::CommandEvent;

pub struct NativeTransformer {
  #[allow(dead_code)]
  platform: &'static Platform,
  #[allow(dead_code)]
  settings: &'static Settings,
}

impl NativeTransformer {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings }
  }

  pub fn init(&mut self) {
    log!("<$>NativeTransformer</>: Init");
  }
}

impl Transformer for NativeTransformer {
  async fn transform(&self, _event: &CommandEvent, _target: &InputType) -> String {
    let result = String::from("Native 🔥🔥🔥🔥🔥🔥🔥 скрипт");

    log!("<$>NativeTransformer</>: Result: <i+>{}</>", result);

    result
  }
}
