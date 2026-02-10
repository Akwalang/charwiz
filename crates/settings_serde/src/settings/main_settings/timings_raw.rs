use serde::Deserialize;

use settings_core::settings::main_settings::Timings;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimingsRaw {
  pub forget_timeout: u64,
}

impl Into<Timings> for TimingsRaw {
  fn into(self) -> Timings {
    Timings {
      forget_timeout: self.forget_timeout,
    }
  }
}
