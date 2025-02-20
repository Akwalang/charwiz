use std::error::Error;

use crate::services::platform::KeyboardLayout;

use crate::services::transformer::native::Native;
use crate::services::transformer::plugin::Plugin;

use crate::services::config::settings::SettingsAction;

pub struct Transformer {
  pub native: Native,
  pub plugin: Plugin,
}

impl Transformer {
  pub fn new() -> Self {
    let native = Native::new();
    let plugin = Plugin::new();

    Self { native, plugin }
  }

  pub fn execute(
    &self,
    value: String,
    action: &SettingsAction,
    kbl_before: &KeyboardLayout,
    kbl_after: &KeyboardLayout,
  ) -> Result<String, Box<dyn Error>> {
    let is_native = Native::is_applicable(action);
    let is_plugin = Plugin::is_applicable(action);

    let lines = value.split("\r\n")
      .enumerate()
      .map(|(idx, line)| {
        println!("line: {}", line);

        if is_native {
          self.native.apply(&line, idx, action, kbl_before, kbl_after)
        } else if is_plugin {
          self.plugin.apply(&line, idx, action, kbl_before, kbl_after)
        } else {
          Err(Box::<dyn Error>::from("Handler not found"))
        }
      })
      .collect::<Result<Vec<String>, Box<dyn Error>>>()?;

    Ok(lines.join("\r\n"))
  }
}
