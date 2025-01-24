mod enums;
pub use enums::PlatformAction;

mod structs;
pub use structs::KeyboardLayout;

mod traits;
pub use traits::{Platform as PlatformTrait};

mod utils;
use utils::replace_languages;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::Windows as Platform;
