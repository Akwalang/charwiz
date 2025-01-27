use super::utils::{get_config, ActionConfig};

pub struct Config {}

impl Config {
  pub fn get_actions() -> &'static Vec<ActionConfig> {
    let config = get_config();

    &config.actions
  }
}
