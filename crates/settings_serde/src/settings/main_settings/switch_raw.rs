use serde::Deserialize;

use settings_core::settings::main_settings::Switch;

use crate::structs::{TransformerRaw, InjectorRaw};

#[derive(Deserialize)]
pub struct SwitchRaw {
  pub text: String,
  pub transformer: TransformerRaw,
  pub injector: InjectorRaw,
}

impl Into<Switch> for SwitchRaw {
  fn into(self) -> Switch {
    Switch {
      text: self.text.chars().collect(),
      transformer: self.transformer.into(),
      injector: self.injector.into(),
    }
  }
}
