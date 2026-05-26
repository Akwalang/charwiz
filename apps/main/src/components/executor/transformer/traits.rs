use crate::Platform;
use crate::Settings;

use crate::common::events::CommandEvent;

use crate::components::executor::enums::InputType;

pub trait Transformer {
  fn new(platform: &'static Platform, settings: &'static Settings) -> Self;

  async fn transform(&self, event: &CommandEvent, target: &InputType, context: &Option<String>) -> InputType;
}
