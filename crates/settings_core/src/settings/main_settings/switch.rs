use crate::structs::{Transformer, Injector};

#[derive(Debug, Clone)]
pub struct Switch {
  pub text: Vec<char>,
  pub transformer: Transformer,
  pub injector: Injector,
}
