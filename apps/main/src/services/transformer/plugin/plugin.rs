use std::fs;
use std::path::Path;

use rlua::{Lua, LuaOptions, Table, StdLib};

use crate::constants::PLUGINS_FOLDER;

use crate::services::config::settings::SettingsAction;
use crate::services::platform::KeyboardLayout;

pub struct Plugin {
  lua: Option<Lua>,
}

impl Plugin {
  pub fn is_applicable(_action: &SettingsAction) -> bool {
    true
  }

  pub fn new() -> Plugin {
    let lua = Self::initialize_lua_scripts();
    let lua = lua.or::<Option<Lua>>(Ok(None)).unwrap();

    if lua.is_none() {
      println!("Error: Lua scripts could not be loaded");
    }

    Plugin { lua }
  }

  fn initialize_lua_scripts() -> Result<Option<Lua>, Box<dyn std::error::Error>> {
    let safe_libs = StdLib::ALL ^ (StdLib::OS | StdLib::IO | StdLib::DEBUG | StdLib::PACKAGE);

    let lua = Lua::new_with(safe_libs, LuaOptions::new())?;

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

  pub fn apply(
    &self,
    value: &str,
    index: usize,
    action: &SettingsAction,
    kbl_before: &KeyboardLayout,
    kbl_after: &KeyboardLayout,
  ) -> Result<String, Box<dyn std::error::Error>> {
    if self.lua.is_none() { return Ok(value.to_owned()); }

    let lua = self.lua.as_ref().unwrap();

    let handler: Result<rlua::Function, rlua::Error> = lua.globals().get(action.handler.to_string());

    if handler.is_err() { return Ok(value.to_owned()); }

    let data = self.prepare_data(value, index, kbl_before, kbl_after)?;

    let result: String = handler.unwrap().call(data)?;

    Ok(result)
  }

  fn prepare_data(
    &self,
    value: &str,
    index: usize,
    before: &KeyboardLayout,
    after: &KeyboardLayout,
  ) -> Result<Table<'_>, Box<dyn std::error::Error>> {
    let lua = self.lua.as_ref().unwrap();

    let data = lua.create_table()?;

    data.set("value", value)?;
    data.set("index", index)?;

    let kbl_before = lua.create_table()?;

    kbl_before.set("id", before.id.clone())?;
    kbl_before.set("name", before.name.clone())?;

    data.set("kbl_before", kbl_before)?;

    let kbl_after = lua.create_table()?;

    kbl_after.set("id", after.id.clone())?;
    kbl_after.set("name", after.name.clone())?;

    data.set("kbl_after", kbl_after)?;

    Ok(data)
  }
}
