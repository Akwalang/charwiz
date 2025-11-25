use rust_logger::*;

use crate::Platform;
use crate::Settings;

use crate::components::transformers::Transformer;

use crate::common::enums::ExecutorType;
use crate::common::events::CommandEvent;

pub struct NativeTransformer {
  platform: &'static Platform,
  settings: &'static Settings,

  r#type: ExecutorType,
}

impl NativeTransformer {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings, r#type: ExecutorType::Native }
  }

  pub fn init(&mut self) {
    log!("<$>NativeTransformer</>: Init");
  }
}

impl Transformer for NativeTransformer {
  fn get_type(&self) -> &ExecutorType {
    &self.r#type
  }

  fn apply(&self, event: CommandEvent) {
    println!("NativeTransformer: {:?}", event);
  }
}
