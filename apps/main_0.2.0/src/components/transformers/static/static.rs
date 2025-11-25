use rust_logger::*;

use crate::Platform;
use crate::Settings;

use crate::components::transformers::Transformer;

use crate::common::enums::ExecutorType;
use crate::common::events::CommandEvent;

pub struct StaticTransformer {
  platform: &'static Platform,
  settings: &'static Settings,

  r#type: ExecutorType,
}

impl StaticTransformer {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings, r#type: ExecutorType::Static }
  }

  pub fn init(&mut self) {
    log!("<$>StaticTransformer</>: Init");
  }
}

impl Transformer for StaticTransformer {
  fn get_type(&self) -> &ExecutorType {
    &self.r#type
  }

  fn apply(&self, event: CommandEvent) {
    println!("StaticTransformer: {:?}", event);
  }
}
