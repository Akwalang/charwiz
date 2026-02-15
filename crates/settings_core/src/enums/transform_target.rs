#[derive(Debug, Clone, PartialEq)]
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
