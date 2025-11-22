use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutorType {
  Native, // Rust converters
  Plugin, // Lua converters
  Static, // Replace by static value
}
