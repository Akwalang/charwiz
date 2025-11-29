use crate::common::enums::KeyboardLayoutEnum;
use crate::common::structs::KeyboardEventSnapshot;

pub enum Action {
  CopyToClipboard,
  PasteFromClipboard,
  SwitchKeyboardLayout(KeyboardLayoutEnum),
  EmitEvents(Vec<KeyboardEventSnapshot>),
  TypeText(String),
}
