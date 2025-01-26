use std::fs;
use std::path::Path;

use rlua::{Lua, Result};

use crate::constants::PLUGINS_FOLDER;

pub struct Plugins {
  lua: Option<Lua>,
}

impl Plugins {
  pub fn new() -> Plugins {
    let lua = Self::initialize_lua_scripts();
    let lua = lua.or::<Option<Lua>>(Ok(None)).unwrap();

    if lua.is_none() {
      println!("Error: Lua scripts could not be loaded");
    }

    Plugins { lua }
  }

  fn initialize_lua_scripts() -> Result<Option<Lua>> {
    let lua = Lua::new();

    let dir = Path::new(PLUGINS_FOLDER);

    println!("Loading Lua scripts from: {:?}\n", dir);

    if !dir.exists() {
      println!("Error: Lua scripts directory does not exist");
      return Ok(None);
    }

    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();

      if path.extension().and_then(|s| s.to_str()) == Some("lua") {
        println!("Loading: {:?}", path);

        let script = fs::read_to_string(&path)?;

        lua.load(&script).exec()?;
      }
    }

    Ok(Some(lua))
  }

  pub fn run(&self, script: &str, value: &str) -> Result<String> {
    let lua = self.lua.as_ref().unwrap();

    let func: rlua::Function = lua.globals().get(script)?;

    let result: String = func.call(value)?;

    Ok(result)
  }
}
