use std::fs;
use std::path::Path;

use rust_logger::*;

use mlua::{Lua, LuaOptions, StdLib, Table, Function, Error};

use crate::Platform;
use crate::Settings;

use super::super::traits::Transformer;
use super::utils;

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
    let safe_libs = StdLib::ALL ^ (/*StdLib::OS | */StdLib::IO | StdLib::DEBUG | StdLib::PACKAGE);

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
    event: &CommandEvent,
    target: &InputType,
  ) -> anyhow::Result<Table<'_>> {
    let lua = self.lua.as_ref().unwrap();

    let data = lua.create_table()?;

    data.set("target", utils::injector_to_lua(&lua, &event.injector)?)?;

    Ok(data)
  }
}

impl Transformer for PluginTransformer {
  async fn transform(&self, event: &CommandEvent, target: &InputType) -> InputType {
    let Some(lua) = self.lua.as_ref() else {
      warn!("<$>PluginTransformer</>: Lua not initialized");
      return target.clone();
    };

    let Ok(handler): Result<Function, Error> = lua.globals().get(event.executor.value.clone()) else {
      warn!("<$>PluginTransformer</>: Lua function not found");
      return target.clone();
    };

    let Ok(data) = self.prepare_data(event, target) else {
      warn!("<$>PluginTransformer</>: Can't transform event data to Lua table");
      return target.clone();
    };

    let Ok(result): Result<String, Error> = handler.call(data) else {
      warn!("<$>PluginTransformer</>: Error during Lua function execution");
      return target.clone();
    };

    log!("<$>PluginTransformer</>: Result: <i+>{}</>", result);

    InputType::Text(result)
  }
}
