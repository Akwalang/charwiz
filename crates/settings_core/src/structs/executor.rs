use crate::enums::ExecutorTypeEnum;

#[derive(Debug, Clone)]
pub struct Executor {
  pub r#type: ExecutorTypeEnum,
  pub value: String,
}
