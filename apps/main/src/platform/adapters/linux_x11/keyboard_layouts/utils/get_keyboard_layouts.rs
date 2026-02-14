use std::process::Command;

use crate::platform::common::structs::KeyboardLayoutItem;

pub fn get_keyboard_layouts() -> Vec<KeyboardLayoutItem> {
  let Ok(output) = Command::new("setxkbmap")
    .arg("-query")
    .output() else {
      return vec![];
    };

  let stdout = String::from_utf8_lossy(&output.stdout);

  for line in stdout.lines() {
    if line.trim_start().starts_with("layout:") {
      let res: Vec<String> = line
        .split(':')
        .nth(1)
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    }
  }

  vec![]
}
