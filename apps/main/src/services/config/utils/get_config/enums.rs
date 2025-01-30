#[derive(Debug, Clone)]
pub enum ActionTarget {
  All,
  Line,
  Word,
  Selection,
  Clipboard,
  None,
}
