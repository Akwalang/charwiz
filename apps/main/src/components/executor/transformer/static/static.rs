#[cfg(feature = "logger")]
use logger::*;

use crate::Platform;
use crate::Settings;

use super::super::traits::Transformer;

use crate::components::executor::enums::InputType;

use crate::common::events::CommandEvent;

pub struct StaticTransformer {
  #[allow(dead_code)]
  platform: &'static Platform,
  #[allow(dead_code)]
  settings: &'static Settings,
}

impl StaticTransformer {
  pub fn init(&mut self) {
    #[cfg(feature = "logger")]
    log!("<$>Static Transformer</>: Init");
  }
}

impl Transformer for StaticTransformer {
  fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings }
  }

  async fn transform(&self, event: &CommandEvent, _target: &InputType, context: &Option<String>) -> InputType {
    let result = event.transformer.value.to_owned();

    #[cfg(feature = "logger")]
    log!("<$>Static Transformer</>: Result: <i+>{}</>", result);

    InputType::Text(result)
  }
}
