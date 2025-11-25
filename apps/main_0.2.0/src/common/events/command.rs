use crate::common::structs::KeyboardEventSnapshot;
use crate::settings::structs::{Executor, Injector};

#[derive(Debug, Clone)]
pub struct CommandEvent {
  pub char_stack: Vec<char>,
  pub event_stack: Vec<KeyboardEventSnapshot>,

  pub executor: Executor,
  pub injector: Injector,
}
