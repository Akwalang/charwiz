mod enums;
mod structs;
mod settings;

pub use enums::HotKeyAction;
pub use structs::HotKey;

pub struct Settings {}

impl Settings {
  pub fn get_hotkeys() -> &'static Vec<HotKey> {
    settings::get_hotkeys()
  }
}
