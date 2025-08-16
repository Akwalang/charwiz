use rust_logger::*;

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
  ) -> anyhow::Result<String> {
    let is_native = Native::is_applicable(action);
    let is_plugin = Plugin::is_applicable(action);

    let lines = value.split("\r\n")
      .enumerate()
      .map(|(idx, line)| {
        debug!("line: {}", line);

        if is_native {
          self.native.apply(&line, idx, action, kbl_before, kbl_after)
            .map_err(|e| anyhow::anyhow!("{e}"))
        } else if is_plugin {
          self.plugin.apply(&line, idx, action, kbl_before, kbl_after)
            .map_err(|e| anyhow::anyhow!("{e}"))
        } else {
          Err(anyhow::anyhow!("Handler not found"))
        }
      })
      .collect::<anyhow::Result<Vec<String>>>()?;

    Ok(lines.join("\r\n"))
  }
}
