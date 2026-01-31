use serde::Deserialize;

use crate::settings::main_settings::structs::{
  Timings,
  HotkeyRaw, HotKey,
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
  pub commands: Vec<CommandRaw>,
  pub tooltips: Vec<TooltipRaw>,
}

#[derive(Debug, Default)]
pub struct Settings {
  pub timings: Timings,
  pub auto_converts: Vec<AutoConvert>,
  pub hotkeys: Vec<HotKey>,
  pub commands: Vec<Command>,
  pub tooltips: Vec<Tooltip>,
}

impl Into<Settings> for SettingsRaw {
  fn into(self) -> Settings {
    Settings {
      timings: self.timings,
      auto_converts: self.auto_converts.into_iter().map(Into::into).collect(),
      hotkeys: self.hotkeys.into_iter().map(Into::into).collect(),
      commands: self.commands.into_iter().map(Into::into).collect(),
      tooltips: self.tooltips.into_iter().map(Into::into).collect(),
    }
  }
}
