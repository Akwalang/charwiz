use std::rc::Rc;

use serde::Deserialize;

use crate::settings::main_settings::structs::{
  Timings,
  HotkeyRaw, HotKey,
  SymbolRaw, Symbol,
  AutoConvertRaw, AutoConvert,
  CommandRaw, Command,
  TooltipRaw, Tooltip,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsRaw {
  pub timings: Timings,
  pub auto_converts: Vec<AutoConvertRaw>,
  pub hotkeys: Vec<HotkeyRaw>,
  pub symbols: Vec<SymbolRaw>,
  pub commands: Vec<CommandRaw>,
  pub tooltips: Vec<TooltipRaw>,
}

#[derive(Debug, Default)]
pub struct Settings {
  pub timings: Timings,
  pub auto_converts: Rc<Vec<AutoConvert>>,
  pub hotkeys: Rc<Vec<HotKey>>,
  pub symbols: Rc<Vec<Symbol>>,
  pub commands: Rc<Vec<Command>>,
  pub tooltips: Rc<Vec<Tooltip>>,
}

impl Into<Settings> for SettingsRaw {
  fn into(self) -> Settings {
    Settings {
      timings: self.timings,
      auto_converts: Rc::new(self.auto_converts.into_iter().map(Into::into).collect()),
      hotkeys: Rc::new(self.hotkeys.into_iter().map(Into::into).collect()),
      symbols: Rc::new(self.symbols.into_iter().map(Into::into).collect()),
      commands: Rc::new(self.commands.into_iter().map(Into::into).collect()),
      tooltips: Rc::new(self.tooltips.into_iter().map(Into::into).collect()),
    }
  }
}
