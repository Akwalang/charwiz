use crate::settings::structs::{AutoConvert, Command, HotKey};

#[derive(Debug, Clone)]
pub enum CommandData {
  AutoConvert(Box<AutoConvert>),
  Command(Box<Command>),
  HotKey(Box<HotKey>),
}

#[derive(Debug, Clone)]
pub struct CommandEvent {
  pub command: CommandData,
}
