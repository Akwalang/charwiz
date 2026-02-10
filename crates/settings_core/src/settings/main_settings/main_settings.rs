use std::rc::Rc;

use super::Timings;
use super::HotKey;
use super::Symbol;
use super::Switch;
use super::Command;
use super::Tooltip;

#[derive(Debug, Default)]
pub struct MainSettings {
  pub timings: Timings,
  pub switches: Rc<Vec<Switch>>,
  pub hotkeys: Rc<Vec<HotKey>>,
  pub symbols: Rc<Vec<Symbol>>,
  pub commands: Rc<Vec<Command>>,
  pub tooltips: Rc<Vec<Tooltip>>,
}
