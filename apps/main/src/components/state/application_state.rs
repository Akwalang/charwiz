#[cfg(feature = "logger")]
use logger::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ApplicationStatus {
  Starting,
  Listening,
  Executing,
  Disabled,
}

pub struct ApplicationState {
  status: ApplicationStatus,
}

impl ApplicationState {
  pub fn new() -> Self {
    ApplicationState {
      status: ApplicationStatus::Starting,
    }
  }

  pub fn set_status(&mut self, status: ApplicationStatus) {
    #[cfg(feature = "logger")]
    log!("<$>State</>: Status changed from <&>{:?}</> to <&>{:?}</>", self.status, status);

    self.status = status;
  }

  pub fn is_disabled(&self) -> bool {
    self.status == ApplicationStatus::Disabled
  }

  pub fn is_executing(&self) -> bool {
    self.status == ApplicationStatus::Executing
  }
}
