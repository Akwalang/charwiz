#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformerTypeEnum {
  Native, // Rust converters
  Plugin, // Lua converters
  Static, // Replace by static value
}

impl std::fmt::Display for TransformerTypeEnum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TransformerTypeEnum::Native => write!(f, "native"),
      TransformerTypeEnum::Plugin => write!(f, "plugin"),
      TransformerTypeEnum::Static => write!(f, "static"),
    }
  }
}
