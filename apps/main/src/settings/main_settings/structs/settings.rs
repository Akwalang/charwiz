use std::rc::Rc;

use serde::Deserialize;

use crate::settings::main_settings::structs::{
  Timings,
  HotkeyRaw, HotKey,
  SymbolRaw, Symbol,
  SwitchRaw, Switch,
  CommandRaw, Command,
  TooltipRaw, Tooltip,
};

#[derive(Deserialize)]
pub struct SettingsRaw {
  pub timings: Timings,
  pub switches: Vec<SwitchRaw>,
  pub hotkeys: Vec<HotkeyRaw>,
  pub symbols: Vec<SymbolRaw>,
  pub commands: Vec<CommandRaw>,
  pub tooltips: Vec<TooltipRaw>,
}

#[derive(Debug, Default)]
pub struct Settings {
  pub timings: Timings,
  pub switches: Rc<Vec<Switch>>,
  pub hotkeys: Rc<Vec<HotKey>>,
  pub symbols: Rc<Vec<Symbol>>,
  pub commands: Rc<Vec<Command>>,
  pub tooltips: Rc<Vec<Tooltip>>,
}

impl Into<Settings> for SettingsRaw {
  fn into(self) -> Settings {
    Settings {
      timings: self.timings,
      switches: Rc::new(self.switches.into_iter().map(Into::into).collect()),
      hotkeys: Rc::new(self.hotkeys.into_iter().map(Into::into).collect()),
      symbols: Rc::new(self.symbols.into_iter().map(Into::into).collect()),
      commands: Rc::new(self.commands.into_iter().map(Into::into).collect()),
      tooltips: Rc::new(self.tooltips.into_iter().map(Into::into).collect()),
    }
  }
}
