mod structs;
pub use structs::{TomlDelay, TomlFile, TomlAction};

mod utils;
pub use utils::read_config_file;
