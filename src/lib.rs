#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use std::panic::PanicInfo;
use std::sync::atomic::{self, AtomicBool};
use std::sync::Once;
use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::{BOOL, TRUE};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use log::{error, info};

static MAIN_THREAD_ONCE: Once = Once::new();
static RUNNING: AtomicBool = AtomicBool::new(true);

mod hooks;
mod luavm;

mod logger {
    use log::LevelFilter;
    use mhw_toolkit::logger::MHWLogger;
    use once_cell::sync::Lazy;

    static LOGGER: Lazy<MHWLogger> = Lazy::new(|| MHWLogger::new(env!("CARGO_PKG_NAME")));

    pub fn init_log() {
        log::set_logger(&*LOGGER).unwrap();
        log::set_max_level(LevelFilter::Debug);
    }
}

pub fn panic_handler(_panic_info: &PanicInfo) {
    let panic_message =
        "A critical plugin failure occurred and the program will shutdown immediately.".to_owned();
    error!("{}\n\n{}", panic_message, _panic_info);
    RUNNING.store(false, atomic::Ordering::SeqCst);
    thread::sleep(Duration::from_secs(2));

    std::process::exit(1);
}

#[no_mangle]
pub unsafe extern "C" fn Init(lua_state: *mut mlua::lua_State) -> i32 {
    if let Err(e) = luavm::LuaVMManager::get_instance()
        .lock()
        .unwrap()
        .register_lua(lua_state)
    {
        error!("注册Lua虚拟机失败: {}", e);
        return 1;
    };

    0
}

#[no_mangle]
pub unsafe extern "C" fn Deinit(lua_state: *mut mlua::lua_State) -> i32 {
    if let Err(e) = luavm::LuaVMManager::get_instance()
        .lock()
        .unwrap()
        .unregister_lua(lua_state)
    {
        error!("注销Lua虚拟机失败: {}", e);
    };

    0
}

#[no_mangle]
extern "system" fn DllMain(_: usize, call_reason: u32, _: usize) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => MAIN_THREAD_ONCE.call_once(|| {
            logger::init_log();
            std::panic::set_hook(Box::new(panic_handler));
            info!(
                "LuaEngineUtilities plugin loaded. Version: {}",
                env!("CARGO_PKG_VERSION")
            );
        }),
        DLL_PROCESS_DETACH => {
            RUNNING.store(false, atomic::Ordering::SeqCst);
        }
        _ => (),
    }
    TRUE
}
