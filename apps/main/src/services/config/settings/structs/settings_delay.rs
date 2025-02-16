use crate::services::config::file::TomlDelay;

#[derive(Debug)]
pub struct SettingsDelay {
  pub long: u64,
  pub medium: u64,
  pub short: u64,
}

impl Into<SettingsDelay> for TomlDelay {
  fn into(self) -> SettingsDelay {
    let TomlDelay { long, medium, short } = self;

    SettingsDelay { long, medium, short }
  }
}
