use serde_json::to_string;
use mlua::{Lua, Table};

use crate::components::executor::enums::InputType;
use crate::settings::structs::Injector;

pub fn injector_to_lua<'a>(lua: &'a Lua, injector: &Injector) -> anyhow::Result<Table<'a>> {
  let lua_data = lua.create_table()?;

  lua_data.set("user_input_cleanup", to_string(&injector.user_input_cleanup)?)?;
  lua_data.set("keyboard_state_cleanup", to_string(&injector.keyboard_state_cleanup)?)?;
  lua_data.set("target", to_string(&injector.target)?)?;
  lua_data.set("method", to_string(&injector.method)?)?;
  lua_data.set("layout_before", to_string(&injector.layout_before)?)?;
  lua_data.set("layout_after", to_string(&injector.layout_after)?)?;

  Ok(lua_data)
}

// pub fn target_to_lua<'a>(lua: &'a Lua, target: &InputType) -> anyhow::Result<Table<'a>> {
//   let lua_data = lua.create_table()?;

//   lua_data.set("user_input_cleanup", to_string(&target.user_input_cleanup)?)?;
//   lua_data.set("keyboard_state_cleanup", to_string(&injector.keyboard_state_cleanup)?)?;
//   lua_data.set("target", to_string(&injector.target)?)?;
//   lua_data.set("method", to_string(&injector.method)?)?;
//   lua_data.set("layout", to_string(&injector.layout)?)?;

//   Ok(lua_data)
// }
