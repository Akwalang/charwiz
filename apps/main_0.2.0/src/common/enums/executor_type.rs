use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutorTypeEnum {
  Native, // Rust converters
  Plugin, // Lua converters
  Static, // Replace by static value
}

impl std::fmt::Display for ExecutorTypeEnum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ExecutorTypeEnum::Native => write!(f, "native"),
      ExecutorTypeEnum::Plugin => write!(f, "plugin"),
      ExecutorTypeEnum::Static => write!(f, "static"),
    }
  }
}
