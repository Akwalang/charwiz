use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timings {
  pub forget_timeout: u64,
  pub key_action_delay: u64,
}
