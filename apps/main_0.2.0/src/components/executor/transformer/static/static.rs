use rust_logger::*;

use crate::Platform;
use crate::Settings;

use super::super::traits::Transformer;

use crate::components::executor::enums::InputType;

use crate::common::events::CommandEvent;
pub struct StaticTransformer {
  platform: &'static Platform,
  settings: &'static Settings,
}

impl StaticTransformer {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings }
  }

  pub fn init(&mut self) {
    log!("<$>StaticTransformer</>: Init");
  }
}

impl Transformer for StaticTransformer {
  async fn transform(&self, event: &CommandEvent, _target: &InputType) -> String {
    let result = event.executor.value.to_owned();

    log!("<$>StaticTransformer</>: Result: <i+>{}</>", result);

    result
  }
}
