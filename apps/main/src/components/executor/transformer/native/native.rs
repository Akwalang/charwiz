#[cfg(feature = "logger")]
use logger::*;

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
  pub fn init(&mut self) {
    #[cfg(feature = "logger")]
    log!("<$>Native Transformer</>: Init");
  }
}

impl Transformer for NativeTransformer {
  fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings }
  }

  async fn transform(&self, event: &CommandEvent, target: &InputType) -> InputType {
    match event.transformer.value.as_str() {
      "invert_case" => methods::invert_case(target),
      "convert_layout" => methods::convert_layout(self.platform, self.settings, event, target),
      "none" => target.clone(),
      _ => InputType::Text("Transformer not found".to_owned()),
    }
  }
}
