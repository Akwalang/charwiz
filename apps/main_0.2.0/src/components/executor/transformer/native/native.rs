use rust_logger::*;

use crate::Platform;
use crate::Settings;

use super::methods;
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
  async fn transform(&self, event: &CommandEvent, target: &InputType) -> InputType {
    match event.executor.value.as_str() {
      "invert_case" => methods::invert_case(target),
      _ => InputType::Text("Transformer not found".to_owned()),
    }
  }
}
