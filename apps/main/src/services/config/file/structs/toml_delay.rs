use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TomlDelay {
    pub long: u64,
    pub medium: u64,
    pub short: u64,
}
