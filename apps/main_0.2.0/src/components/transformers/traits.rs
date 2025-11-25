use crate::common::enums::ExecutorTypeEnum;
use crate::common::events::CommandEvent;

pub trait Transformer {
  fn get_type(&self) -> &ExecutorTypeEnum;
  fn transform(&self, event: &CommandEvent) -> String;
}
