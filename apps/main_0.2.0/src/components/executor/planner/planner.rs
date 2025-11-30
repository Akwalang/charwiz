use crate::platform::Platform;
use crate::settings::Settings;

pub struct Planner {
  platform: &'static Platform,
  settings: &'static Settings,
}

impl Planner {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    Self { platform, settings }
  }


}
