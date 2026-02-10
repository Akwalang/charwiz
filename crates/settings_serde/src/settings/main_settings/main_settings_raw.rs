use std::rc::Rc;

use serde::Deserialize;

use settings_core::settings::main_settings::MainSettings;

use super::TimingsRaw;
use super::SwitchRaw;
use super::HotkeyRaw;
use super::SymbolRaw;
use super::CommandRaw;
use super::TooltipRaw;

#[derive(Deserialize)]
pub struct MainSettingsRaw {
  pub timings: TimingsRaw,
  pub switches: Vec<SwitchRaw>,
  pub hotkeys: Vec<HotkeyRaw>,
  pub symbols: Vec<SymbolRaw>,
  pub commands: Vec<CommandRaw>,
  pub tooltips: Vec<TooltipRaw>,
}

impl Into<MainSettings> for MainSettingsRaw {
  fn into(self) -> MainSettings {
    MainSettings {
      timings: self.timings.into(),
      switches: Rc::new(self.switches.into_iter().map(Into::into).collect()),
      hotkeys: Rc::new(self.hotkeys.into_iter().map(Into::into).collect()),
      symbols: Rc::new(self.symbols.into_iter().map(Into::into).collect()),
      commands: Rc::new(self.commands.into_iter().map(Into::into).collect()),
      tooltips: Rc::new(self.tooltips.into_iter().map(Into::into).collect()),
    }
  }
}
