use crate::settings::structs::{AutoConvert, Command, HotKey, Executor};

#[derive(Debug, Clone)]
pub enum CommandData {
  AutoConvert(Box<AutoConvert>),
  Command(Box<Command>),
  HotKey(Box<HotKey>),
}

#[derive(Debug, Clone)]
pub struct CommandEvent {
  pub command: CommandData,
  pub executor: Executor,
}
