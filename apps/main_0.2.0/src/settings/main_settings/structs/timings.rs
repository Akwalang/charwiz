use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Timings {
  pub forget_timeout: u32,
  pub long_delay: u32,
  pub medium_delay: u32,
  pub short_delay: u32,
}
