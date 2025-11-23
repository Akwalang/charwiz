use serde::Deserialize;

use crate::settings::main_settings::structs::{
  Timings,
  Executor,
  HotkeyRaw, HotKey,
  AutoConvert,
  Command,
  Tooltip,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsRaw {
  pub timings: Timings,
  pub executors: Vec<Executor>,
  pub auto_converts: Vec<AutoConvert>,
  pub hotkeys: Vec<HotkeyRaw>,
  pub commands: Vec<Command>,
  pub tooltips: Vec<Tooltip>,
}

#[derive(Debug, Default)]
pub struct Settings {
  pub timings: Timings,
  pub executors: Vec<Executor>,
  pub auto_converts: Vec<AutoConvert>,
  pub hotkeys: Vec<HotKey>,
  pub commands: Vec<Command>,
  pub tooltips: Vec<Tooltip>,
}

impl From<SettingsRaw> for Settings {
  fn from(raw: SettingsRaw) -> Self {
    Self {
      timings: raw.timings,
      executors: raw.executors,
      auto_converts: raw.auto_converts,
      hotkeys: raw.hotkeys.into_iter().map(|hk| hk.into()).collect(),
      commands: raw.commands,
      tooltips: raw.tooltips,
    }
  }
}
