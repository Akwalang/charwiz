#[derive(Debug, Clone)]
pub enum KeyboardLayoutEnum {
  Previous,
  Current,
  Next,
  Direct(String),
}
