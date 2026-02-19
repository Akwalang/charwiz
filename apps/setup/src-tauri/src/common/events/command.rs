use settings_core::structs::{Transformer, Injector, KeyboardSnapshot};

#[derive(Debug, Clone)]
pub struct CommandEvent {
  pub char_stack: Vec<char>,
  pub event_stack: Vec<KeyboardSnapshot>,

  pub transformer: Transformer,
  pub injector: Injector,
}
