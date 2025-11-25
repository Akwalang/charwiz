use rust_logger::*;

use crate::Platform;
use crate::Settings;

use crate::components::transformers::Transformer;

use crate::common::enums::ExecutorTypeEnum;
use crate::common::events::CommandEvent;

pub struct StaticTransformer {
  platform: &'static Platform,
  settings: &'static Settings,

  r#type: ExecutorTypeEnum,
}

impl StaticTransformer {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings, r#type: ExecutorTypeEnum::Static }
  }

  pub fn init(&mut self) {
    log!("<$>StaticTransformer</>: Init");
  }
}

impl Transformer for StaticTransformer {
  fn get_type(&self) -> &ExecutorTypeEnum {
    &self.r#type
  }

  fn transform(&self, event: &CommandEvent, _target: &str) -> String {
    let result = event.executor.value.to_owned();

    log!("<$>StaticTransformer</>: Result: <i+>{}</>", result);

    result
  }
}
