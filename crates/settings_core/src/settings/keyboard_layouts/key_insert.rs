use crate::structs::KeyboardModifiers;

#[derive(Debug)]
pub struct KeyInsert {
  pub r#char: char,
  pub modifiers: KeyboardModifiers,
}
