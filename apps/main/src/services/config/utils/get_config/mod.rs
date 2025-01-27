mod enums;
pub use enums::ActionTarget;

mod structs;
pub use structs::{ConfigFile, ActionConfig};

mod get_config;
pub use get_config::get_config;
