use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutorType {
  Native, // Rust converters
  Plugin, // Lua converters
  Static, // Replace by static value
}

impl std::fmt::Display for ExecutorType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ExecutorType::Native => write!(f, "native"),
      ExecutorType::Plugin => write!(f, "plugin"),
      ExecutorType::Static => write!(f, "static"),
    }
  }
}
