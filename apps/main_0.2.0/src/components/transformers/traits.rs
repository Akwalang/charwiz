use crate::common::enums::ExecutorType;
use crate::common::events::CommandEvent;

pub trait Transformer {
  fn get_type(&self) -> &ExecutorType;
  fn apply(&self, event: CommandEvent);
}
