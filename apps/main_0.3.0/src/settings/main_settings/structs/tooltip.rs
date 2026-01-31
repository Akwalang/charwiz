use serde::Deserialize;

use crate::common::enums::TooltipTypeEnum;
use crate::common::structs::{Executor, Injector};

use super::super::super::raw::enums::TooltipTypeRawEnum;
use super::super::super::raw::structs::{ExecutorRaw, InjectorRaw};

#[derive(Deserialize)]
pub struct TooltipRaw {
  pub id: String,
  pub items: Vec<TooltipItemRaw>,
}

#[derive(Debug, Clone)]
pub struct Tooltip {
  pub id: String,
  pub items: Vec<TooltipItem>,
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

#[derive(Debug, Clone)]
pub struct TooltipItem {
  pub settings: TooltipSettings,
  pub executor: Executor,
  pub injector: Injector,
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

#[derive(Debug, Clone)]
pub struct TooltipSettings {
  pub r#type: TooltipTypeEnum,
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
