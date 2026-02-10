use settings_core::structs::{Executor, Injector, KeyboardSnapshot};

#[derive(Debug, Clone)]
pub struct CommandEvent {
  pub current_snapshot: KeyboardSnapshot,

  pub char_stack: Vec<char>,
  pub event_stack: Vec<KeyboardSnapshot>,

  pub executor: Executor,
  pub injector: Injector,
}
