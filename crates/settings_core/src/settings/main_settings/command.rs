use crate::structs::{Transformer, Injector};

#[derive(Debug, Clone)]
pub struct Command {
  pub cmd: Vec<char>,
  pub transformer: Transformer,
  pub injector: Injector,
}
