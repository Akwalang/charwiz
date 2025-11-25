use std::fs;
use std::path::Path;

use rust_logger::*;

use mlua::{Lua, LuaOptions, Table, StdLib};

use crate::components::transformers::Transformer;

use crate::Platform;
use crate::Settings;

use crate::common::enums::ExecutorTypeEnum;
use crate::common::events::CommandEvent;

use crate::constants::PLUGINS_FOLDER;

pub struct PluginTransformer {
  platform: &'static Platform,
  settings: &'static Settings,

  r#type: ExecutorTypeEnum,
  lua: Option<Lua>,
}

impl PluginTransformer {
  pub fn new(platform: &'static Platform, settings: &'static Settings) -> Self {
    PluginTransformer { platform, settings, r#type: ExecutorTypeEnum::Plugin, lua: None }
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

      if path.extension().and_then(|s| s.to_str()) == Some("lua") {
        log!("<$>PluginTransformer</>: Loading script: <i&>{}</>", path.to_str().unwrap().replace("\\", "/"));

        let script = fs::read_to_string(&path)?;

        lua.load(&script).exec()?;
      }
    }

    Ok(Some(lua))
  }
}

impl Transformer for PluginTransformer {
  fn get_type(&self) -> &ExecutorTypeEnum {
    &self.r#type
  }

  fn transform(&self, event: &CommandEvent, target: &str) -> String {
    let result = String::from("Plugin result");

    log!("<$>PluginTransformer</>: Result: <i+>{}</>", result);

    result
  }
}
