use std::rc::Rc;

use serde::Deserialize;

use settings_core::settings::main_settings::Settings;

#[derive(Deserialize)]
pub struct SettingsRaw {
  pub timings: Timings,
  pub switches: Vec<SwitchRaw>,
  pub hotkeys: Vec<HotkeyRaw>,
  pub symbols: Vec<SymbolRaw>,
  pub commands: Vec<CommandRaw>,
  pub tooltips: Vec<TooltipRaw>,
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
