use crate::services::tray::enums::TrayAction;

pub struct MenuItem {
  pub name: &'static str,
  pub action: TrayAction,
}

impl MenuItem {
  pub fn new(name: &'static str, action: TrayAction) -> MenuItem {
    MenuItem { name, action }
  }
}
