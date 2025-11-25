use rust_logger::*;

use crate::Platform;
use crate::Settings;

use crate::components::transformers::Transformer;

use crate::common::enums::ExecutorTypeEnum;
use crate::common::events::CommandEvent;

pub struct NativeTransformer {
  platform: &'static Platform,
  settings: &'static Settings,

  r#type: ExecutorTypeEnum,
}

impl NativeTransformer {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings, r#type: ExecutorTypeEnum::Native }
  }

  pub fn init(&mut self) {
    log!("<$>NativeTransformer</>: Init");
  }
}

impl Transformer for NativeTransformer {
  fn get_type(&self) -> &ExecutorTypeEnum {
    &self.r#type
  }

  fn transform(&self, event: &CommandEvent, target: &str) -> String {
    let result = String::from("Native result");

    log!("<$>NativeTransformer</>: Result: <i+>{}</>", result);

    result
  }
}
