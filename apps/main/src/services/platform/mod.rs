mod structs;
pub use structs::KeyboardLayout;

mod traits;
pub use traits::PlatformTrait;

mod utils;
pub use utils::normalize_language_name;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{Windows as Platform, Clipboard};
