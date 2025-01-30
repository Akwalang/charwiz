use serde::Deserialize;
use rdev::Key;

use super::ActionTarget;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
  pub actions: Vec<ActionConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ActionConfig {
  #[serde(deserialize_with = "deserialize_key")]
  pub keys: Vec<Key>,
  #[serde(deserialize_with = "deserialize_target")]
  pub target: ActionTarget,
  #[serde(default)]
  pub switch_keyboard_layout: bool,
  #[serde(default)]
  pub keep_selection: bool,
  pub handler: String,
}

fn deserialize_key<'de, D>(deserializer: D) -> Result<Vec<Key>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let keys_str: Vec<String> = Vec::deserialize(deserializer)?;
  let mut keys = Vec::new();

  for key_str in keys_str {
    let key = str_to_key(&key_str).map_err(serde::de::Error::custom)?;
    keys.push(key);
  }

  Ok(keys)
}

fn deserialize_target<'de, D>(deserializer: D) -> Result<ActionTarget, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let target_str: String = String::deserialize(deserializer)?;

  match target_str.to_lowercase().as_str() {
    "all" => Ok(ActionTarget::All),
    "line" => Ok(ActionTarget::Line),
    "word" => Ok(ActionTarget::Word),
    "selection" => Ok(ActionTarget::Selection),
    "clipboard" => Ok(ActionTarget::Clipboard),
    "none" => Ok(ActionTarget::None),
    _ => Err(serde::de::Error::custom(format!("Unknown target: {}", target_str))),
  }
}

pub fn str_to_key(s: &str) -> Result<Key, String> {
  match s.to_lowercase().as_str() {
    "alt" => Ok(Key::Alt),
    "altgr" => Ok(Key::AltGr),
    "backspace" => Ok(Key::Backspace),
    "capslock" => Ok(Key::CapsLock),
    "ctrl" => Ok(Key::ControlLeft), // shortcut
    "controlleft" => Ok(Key::ControlLeft),
    "controlright" => Ok(Key::ControlRight),
    "delete" => Ok(Key::Delete),
    "downarrow" => Ok(Key::DownArrow),
    "end" => Ok(Key::End),
    "escape" => Ok(Key::Escape),
    "f1" => Ok(Key::F1),
    "f10" => Ok(Key::F10),
    "f11" => Ok(Key::F11),
    "f12" => Ok(Key::F12),
    "f2" => Ok(Key::F2),
    "f3" => Ok(Key::F3),
    "f4" => Ok(Key::F4),
    "f5" => Ok(Key::F5),
    "f6" => Ok(Key::F6),
    "f7" => Ok(Key::F7),
    "f8" => Ok(Key::F8),
    "f9" => Ok(Key::F9),
    "home" => Ok(Key::Home),
    "leftarrow" => Ok(Key::LeftArrow),
    "metaleft" => Ok(Key::MetaLeft),
    "metaright" => Ok(Key::MetaRight),
    "pagedown" => Ok(Key::PageDown),
    "pageup" => Ok(Key::PageUp),
    "return" => Ok(Key::Return),
    "rightarrow" => Ok(Key::RightArrow),
    "shift" => Ok(Key::ShiftLeft), // shortcut
    "shiftleft" => Ok(Key::ShiftLeft),
    "shiftright" => Ok(Key::ShiftRight),
    "space" => Ok(Key::Space),
    "tab" => Ok(Key::Tab),
    "uparrow" => Ok(Key::UpArrow),
    "printscreen" => Ok(Key::PrintScreen),
    "scrolllock" => Ok(Key::ScrollLock),
    "pause" => Ok(Key::Pause),
    "numlock" => Ok(Key::NumLock),
    "backquote" => Ok(Key::BackQuote),
    "num1" => Ok(Key::Num1),
    "num2" => Ok(Key::Num2),
    "num3" => Ok(Key::Num3),
    "num4" => Ok(Key::Num4),
    "num5" => Ok(Key::Num5),
    "num6" => Ok(Key::Num6),
    "num7" => Ok(Key::Num7),
    "num8" => Ok(Key::Num8),
    "num9" => Ok(Key::Num9),
    "num0" => Ok(Key::Num0),
    "minus" => Ok(Key::Minus),
    "equal" => Ok(Key::Equal),
    "keyq" => Ok(Key::KeyQ),
    "keyw" => Ok(Key::KeyW),
    "keye" => Ok(Key::KeyE),
    "keyr" => Ok(Key::KeyR),
    "keyt" => Ok(Key::KeyT),
    "keyy" => Ok(Key::KeyY),
    "keyu" => Ok(Key::KeyU),
    "keyi" => Ok(Key::KeyI),
    "keyo" => Ok(Key::KeyO),
    "keyp" => Ok(Key::KeyP),
    "leftbracket" => Ok(Key::LeftBracket),
    "rightbracket" => Ok(Key::RightBracket),
    "keya" => Ok(Key::KeyA),
    "keys" => Ok(Key::KeyS),
    "keyd" => Ok(Key::KeyD),
    "keyf" => Ok(Key::KeyF),
    "keyg" => Ok(Key::KeyG),
    "keyh" => Ok(Key::KeyH),
    "keyj" => Ok(Key::KeyJ),
    "keyk" => Ok(Key::KeyK),
    "keyl" => Ok(Key::KeyL),
    "semicolon" => Ok(Key::SemiColon),
    "quote" => Ok(Key::Quote),
    "backslash" => Ok(Key::BackSlash),
    "intlbackslash" => Ok(Key::IntlBackslash),
    "keyz" => Ok(Key::KeyZ),
    "keyx" => Ok(Key::KeyX),
    "keyc" => Ok(Key::KeyC),
    "keyv" => Ok(Key::KeyV),
    "keyb" => Ok(Key::KeyB),
    "keyn" => Ok(Key::KeyN),
    "keym" => Ok(Key::KeyM),
    "comma" => Ok(Key::Comma),
    "dot" => Ok(Key::Dot),
    "slash" => Ok(Key::Slash),
    "insert" => Ok(Key::Insert),
    "kpreturn" => Ok(Key::KpReturn),
    "kpminus" => Ok(Key::KpMinus),
    "kpplus" => Ok(Key::KpPlus),
    "kpmultiply" => Ok(Key::KpMultiply),
    "kpdivide" => Ok(Key::KpDivide),
    "kp0" => Ok(Key::Kp0),
    "kp1" => Ok(Key::Kp1),
    "kp2" => Ok(Key::Kp2),
    "kp3" => Ok(Key::Kp3),
    "kp4" => Ok(Key::Kp4),
    "kp5" => Ok(Key::Kp5),
    "kp6" => Ok(Key::Kp6),
    "kp7" => Ok(Key::Kp7),
    "kp8" => Ok(Key::Kp8),
    "kp9" => Ok(Key::Kp9),
    "kpdelete" => Ok(Key::KpDelete),
    "function" => Ok(Key::Function),
    _ => Err(format!("Unknown key: {}", s)),
  }
}
