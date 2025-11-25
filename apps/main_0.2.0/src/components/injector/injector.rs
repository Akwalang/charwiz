use crate::common::events::CommandEvent;

pub struct Injector {}

impl Injector {
  pub fn new() -> Self {
    Self {}
  }

  pub async fn inject(&self, event: CommandEvent, value: String) {
    
  }
}
