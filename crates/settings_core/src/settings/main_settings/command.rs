use crate::structs::{Executor, Injector};

#[derive(Debug, Clone)]
pub struct Command {
  pub cmd: Vec<char>,
  pub executor: Executor,
  pub injector: Injector,
}
