use serde::Deserialize;

use settings_core::settings::main_settings::{Tooltip, TooltipItem, TooltipSettings};

use crate::enums::TooltipTypeRawEnum;
use crate::structs::{ExecutorRaw, InjectorRaw};

#[derive(Deserialize)]
pub struct TooltipRaw {
  pub id: String,
  pub items: Vec<TooltipItemRaw>,
}

impl Into<Tooltip> for TooltipRaw {
  fn into(self) -> Tooltip {
    Tooltip {
      id: self.id,
      items: self.items.into_iter().map(Into::into).collect(),
    }
  }
}


#[derive(Deserialize)]
pub struct TooltipItemRaw {
  pub settings: TooltipSettingsRaw,
  pub executor: ExecutorRaw,
  pub injector: InjectorRaw,
}

impl Into<TooltipItem> for TooltipItemRaw {
  fn into(self) -> TooltipItem {
    TooltipItem {
      settings: self.settings.into(),
      executor: self.executor.into(),
      injector: self.injector.into(),
    }
  }
}


#[derive(Deserialize)]
pub struct TooltipSettingsRaw {
  pub r#type: TooltipTypeRawEnum,
  pub label: String,
}

impl Into<TooltipSettings> for TooltipSettingsRaw {
  fn into(self) -> TooltipSettings {
    TooltipSettings {
      r#type: self.r#type.into(),
      label: self.label,
    }
  }
}
