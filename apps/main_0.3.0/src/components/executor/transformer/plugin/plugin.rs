use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use rust_logger::*;

use mlua::{Lua, LuaOptions, StdLib, Table, Function, Error};

use crate::Platform;
use crate::Settings;

use super::super::traits::Transformer;

use crate::components::executor::enums::InputType;

use crate::common::events::CommandEvent;

use crate::constants::PLUGINS_FOLDER;

pub struct PluginTransformer {
  platform: &'static Platform,
  settings: &'static Settings,

  lua: Option<Lua>,
}

impl PluginTransformer {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    PluginTransformer { platform, settings, lua: None }
  }

  pub fn init(&mut self) {
    log!("<$>PluginTransformer</>: Init");

    self.load_lua();
  }

  pub fn reload(&mut self) {
    self.load_lua();
  }

  fn load_lua(&mut self) {
    let lua = Self::initialize_lua_scripts().unwrap_or(None);

    if lua.is_none() {
      error!("<$>PluginTransformer</>: Lua scripts could not be loaded");
    }

    self.lua = lua;
  }

  fn initialize_lua_scripts() -> anyhow::Result<Option<Lua>> {
    let safe_libs = StdLib::ALL ^ (StdLib::OS | StdLib::IO | StdLib::DEBUG | StdLib::PACKAGE);

    let lua = Lua::new_with(safe_libs, LuaOptions::new())?;

    let dir = Path::new(PLUGINS_FOLDER);

    log!("<$>PluginTransformer</>: Loading Lua scripts from: <i&>{}</>", dir.to_str().unwrap());

    if !dir.exists() {
      warn!("<$>PluginTransformer</>: Lua scripts directory does not exist");
      return Ok(None);
    }

    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();

      if path.extension().and_then(|s| s.to_str()) != Some("lua") { continue; }

      log!("<$>PluginTransformer</>: Loading script: <i&>{}</>", path.to_str().unwrap().replace("\\", "/"));

      let script = fs::read_to_string(&path)?;

      lua.load(&script).exec()?;
    }

    Ok(Some(lua))
  }

  fn prepare_data(
    &self,
    target: &InputType,
  ) -> anyhow::Result<Table<'_>> {
    let lua = self.lua.as_ref().unwrap();

    let lua_data = lua.create_table()?;

    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards")
      .as_secs();
    
    let InputType::Text(value) = target else {
      anyhow::bail!("Can't get the target value");
    };

    lua_data.set("value", value.to_owned())?;
    lua_data.set("timestamp", format!("{}", timestamp))?;

    Ok(lua_data)
  }
}

impl Transformer for PluginTransformer {
  async fn transform(&self, event: &CommandEvent, target: &InputType) -> InputType {
    let Some(lua) = self.lua.as_ref() else {
      warn!("<$>PluginTransformer</>: Lua not initialized");
      return target.clone();
    };

    if let InputType::Events(_) = target {
      warn!("<$>PluginTransformer</>: Event input is banned");
      return target.clone();
    }

    let Ok(handler): Result<Function, Error> = lua.globals().get(event.executor.value.clone()) else {
      warn!("<$>PluginTransformer</>: Lua function not found");
      return target.clone();
    };

    let Ok(data) = self.prepare_data(target) else {
      warn!("<$>PluginTransformer</>: Can't transform event data to Lua table");
      return target.clone();
    };

    let result = handler.call(data);

    let Ok(result): Result<String, Error> = result else {
      warn!("<$>PluginTransformer</>: Error during Lua function execution\n{}", result.err().unwrap());
      return target.clone();
    };

    log!("<$>PluginTransformer</>: Result:\n<i+>{}</>", result);

    InputType::Text(result)
  }
}
