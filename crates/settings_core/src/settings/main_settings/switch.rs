use crate::structs::{Executor, Injector};

#[derive(Debug, Clone)]
pub struct Switch {
  pub text: Vec<char>,
  pub executor: Executor,
  pub injector: Injector,
}
