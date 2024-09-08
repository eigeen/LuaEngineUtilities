mod library;

use mlua::prelude::*;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

type SharedLuaVMManager = Arc<Mutex<LuaVMManager>>;
static LUA_VM_MANAGER: Lazy<Arc<Mutex<LuaVMManager>>> =
    Lazy::new(|| Arc::new(Mutex::new(LuaVMManager::new())));

pub struct LuaVMManager {
    /// 保存的Lua虚拟机状态
    /// key: *mut mlua::lua_State -> usize
    cores: HashMap<usize, LuaVMCore>,
}

impl LuaVMManager {
    pub fn new() -> Self {
        Self {
            cores: HashMap::new(),
        }
    }

    pub fn get_instance() -> SharedLuaVMManager {
        LUA_VM_MANAGER.clone()
    }

    /// 注册Lua虚拟机，保存状态，扩展功能
    pub fn register_lua(&mut self, lua_state: *mut mlua::lua_State) -> anyhow::Result<()> {
        let lua = unsafe { mlua::Lua::init_from_ptr(lua_state) };
        let core = LuaVMCore::new(lua);
        // 扩展功能
        core.register_basic_modules()?;

        self.cores.insert(lua_state as usize, core);

        Ok(())
    }

    /// 注销Lua虚拟机，释放资源
    pub fn unregister_lua(&mut self, lua_state: *mut mlua::lua_State) -> anyhow::Result<()> {
        if let Some(_core) = self.cores.get_mut(&(lua_state as usize)) {
            // core.unregister();
        };
        self.cores.remove(&(lua_state as usize));
        
        Ok(())
    }
}

pub struct LuaVMCore {
    inner: Arc<Mutex<Lua>>,
}

impl LuaVMCore {
    pub fn new(lua: Lua) -> Self {
        Self {
            inner: Arc::new(Mutex::new(lua)),
        }
    }

    /// 注册基础模块
    ///
    /// 基础模块无额外状态需要维护，无需释放资源
    pub fn register_basic_modules(&self) -> anyhow::Result<()> {
        let lua = self.inner.lock().unwrap();
        let globals = lua.globals();
        globals.set(
            "LuaEngineUtilitiesVersion",
            lua.create_function(|_, ()| Ok(env!("CARGO_PKG_VERSION")))?,
        )?;
        globals.set("Memory", lua.create_userdata(library::memory::Memory)?)?;
        globals.set("Game", lua.create_userdata(library::game::Game)?)?;

        Ok(())
    }
}
