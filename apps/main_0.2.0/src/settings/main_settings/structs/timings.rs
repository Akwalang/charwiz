use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timings {
  pub forgetTimeout: u64,
  pub keyActionDelay: u64,
}
