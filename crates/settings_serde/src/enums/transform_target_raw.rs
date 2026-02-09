use serde::Deserialize;

use settings_core::enums::TransformTargetEnum;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransformTargetRawEnum {
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

impl Into<TransformTargetEnum> for TransformTargetRawEnum {
  fn into(self) -> TransformTargetEnum {
    match self {
      TransformTargetRawEnum::None      => TransformTargetEnum::None,
      TransformTargetRawEnum::Input     => TransformTargetEnum::Input,
      TransformTargetRawEnum::Events    => TransformTargetEnum::Events,
      TransformTargetRawEnum::Command   => TransformTargetEnum::Command,
      TransformTargetRawEnum::Clipboard => TransformTargetEnum::Clipboard,
      TransformTargetRawEnum::Selection => TransformTargetEnum::Selection,
      TransformTargetRawEnum::Word      => TransformTargetEnum::Word,
      TransformTargetRawEnum::Line      => TransformTargetEnum::Line,
      TransformTargetRawEnum::All       => TransformTargetEnum::All,
    }
  }
}
