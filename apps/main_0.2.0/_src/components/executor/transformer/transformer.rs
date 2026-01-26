use crate::platform::Platform;
use crate::settings::Settings;

use crate::components::executor::enums::InputType;

use crate::common::enums::ExecutorTypeEnum;
use crate::common::events::CommandEvent;

use super::traits::{Transformer as TransformerTrait};

use super::native::NativeTransformer;
use super::plugin::PluginTransformer;
use super::r#static::StaticTransformer;

pub struct Transformer {
  native_transformer: NativeTransformer,
  plugin_transformer: PluginTransformer,
  static_transformer: StaticTransformer,
}

impl Transformer {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    let mut native_transformer = NativeTransformer::new(platform, settings);
    let mut plugin_transformer = PluginTransformer::new(platform, settings);
    let mut static_transformer = StaticTransformer::new(platform, settings);

    native_transformer.init();
    plugin_transformer.init();
    static_transformer.init();

    Self {
      native_transformer,
      plugin_transformer,
      static_transformer,
    }
  }

  pub async fn transform(&self, event: &CommandEvent, target: &InputType) -> InputType {
    match event.executor.r#type {
      ExecutorTypeEnum::Native => {
        self.native_transformer.transform(event, target).await
      }
      ExecutorTypeEnum::Plugin => {
        self.plugin_transformer.transform(event, target).await
      }
      ExecutorTypeEnum::Static => {
        self.static_transformer.transform(event, target).await
      }
    }
  }
}
