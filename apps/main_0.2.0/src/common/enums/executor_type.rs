use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutorType {
  Native, // Rust converters
  Plugin, // Lua converters
  Static, // Replace by static value
}
