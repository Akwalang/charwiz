use crate::common::events::CommandEvent;

use crate::components::executor::enums::InputType;

pub trait Transformer {
  async fn transform(&self, event: &CommandEvent, target: &InputType) -> InputType;
}
