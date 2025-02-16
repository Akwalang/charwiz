pub struct Plugin;

// use std::fs;
// use std::path::Path;
// use std::sync::{Arc, OnceLock};

// use async_std::sync::Mutex;
// use rlua::{Lua, LuaOptions, Table, StdLib, Result};

// use crate::constants::PLUGINS_FOLDER;

// use crate::services::config::settings::SettingsAction;
// use crate::services::platform::KeyboardLayout;

// static PLUGIN: OnceLock<Lua> = OnceLock::new();

// fn init_plugin() -> Plugin {
//   Plugin::new()
// }

// pub struct Plugin {
//   lua: Option<Lua>,
// }

// impl Plugin {
//   fn new() -> Plugin {
//     let lua = Self::initialize_lua_scripts();
//     let lua = lua.or::<Option<Lua>>(Ok(None)).unwrap();

//     if lua.is_none() {
//       println!("Error: Lua scripts could not be loaded");
//     }

//     Plugin { lua }
//   }

//   pub fn get_instance() -> &'static Plugin {
//     PLUGIN.get_or_init(|| Mutex::new(Plugin::new()))
//   }

//   fn initialize_lua_scripts() -> Result<Option<Lua>> {
//     let safe_libs = StdLib::ALL ^ (StdLib::OS | StdLib::IO | StdLib::DEBUG | StdLib::PACKAGE);

//     let lua = Lua::new_with(safe_libs, LuaOptions::new())?;

//     let dir = Path::new(PLUGINS_FOLDER);

//     println!("Loading Lua scripts from: {:?}\n", dir);

//     if !dir.exists() {
//       println!("Error: Lua scripts directory does not exist");
//       return Ok(None);
//     }

//     for entry in fs::read_dir(dir)? {
//       let entry = entry?;
//       let path = entry.path();

//       if path.extension().and_then(|s| s.to_str()) == Some("lua") {
//         println!("Loading: {:?}", path);

//         let script = fs::read_to_string(&path)?;

//         lua.load(&script).exec()?;
//       }
//     }

//     Ok(Some(lua))
//   }

//   pub fn apply(
//     &self,
//     value: &str,
//     action: &SettingsAction,
//     kbl_before: &KeyboardLayout,
//     kbl_after: &KeyboardLayout,
//   ) -> Result<Option<String>, Box<dyn std::error::Error>> {
//     let lua = self.lua.as_ref().unwrap();
//     let lua = lua.lock().unwrap();

//     if lua.is_none() { return Ok(None); }

//     let lua = lua.as_ref().unwrap();

//     let handler: Result<rlua::Function> = lua.globals().get(action.handler.to_string());

//     if handler.is_err() { return Ok(None); }

//     let data = self.prepare_data(value, kbl_before, kbl_after)?;

//     let result: String = handler.unwrap().call(data)?;

//     Ok(Some(result))
//   }

//   fn prepare_data(
//     &self,
//     value: &str,
//     before: &KeyboardLayout,
//     after: &KeyboardLayout,
//   ) -> Result<Table<'_>> {
//     let lua = self.lua.as_ref().unwrap();

//     let data = lua.create_table()?;

//     data.set("value", value)?;

//     let kbl_before = lua.create_table()?;

//     kbl_before.set("id", before.id.clone())?;
//     kbl_before.set("name", before.name.clone())?;

//     data.set("kbl_before", kbl_before)?;

//     let kbl_after = lua.create_table()?;

//     kbl_after.set("id", after.id.clone())?;
//     kbl_after.set("name", after.name.clone())?;

//     data.set("kbl_after", kbl_after)?;

//     Ok(data)
//   }
// }
