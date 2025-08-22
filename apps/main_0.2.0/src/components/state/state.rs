use rust_logger::*;

use crate::components::state::{Status, KeyboardEventSnapshot};

pub struct State {
  status: Status,
  char_stack: Vec<char>,
  event_stack: Vec<KeyboardEventSnapshot>,
}

impl State {
  pub fn new() -> Self {
    State {
      status: Status::Starting,
      char_stack: Vec::new(),
      event_stack: Vec::new(),
    }
  }

  pub fn set_status(&mut self, status: Status) {
    log!("<$>State</>: Status changed from <&>{:?}</> to <&>{:?}</>", self.status, status);
  
    self.status = status;
  }

  pub fn is_executing(&self) -> bool {
    self.status == Status::Executing
  }
}
