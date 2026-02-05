#[derive(Debug, Clone)]
pub enum TransformTargetEnum {
  None,
  Input,
  Events,
  Command,
  Clipboard,
  Selection,
  Word,
  Line,
  All,
}
