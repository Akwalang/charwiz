use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;

pub fn read_json<Result: DeserializeOwned>(path: impl AsRef<Path>) -> anyhow::Result<Result> {
  let content = fs::read_to_string(&path)?;
  let value = serde_json::from_str::<Result>(&content)?;

  Ok(value)
}
