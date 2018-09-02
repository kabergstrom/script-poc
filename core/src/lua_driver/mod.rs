use libc::c_char;
use lua_jit_sys::*;
use regex::{Regex, RegexBuilder};
use std::ffi::{CStr, CString};
use LanguageDriver;

pub struct LuaRuntime {
    state: *mut lua_State,
    run_func: ::std::os::raw::c_int,
}
pub fn new_lua_runtime() -> Box<LanguageDriver> {
    return Box::new(LuaRuntime {
        state: ::std::ptr::null_mut(),
        run_func: 0,
    });
}

unsafe fn call_lua_func(state: *mut lua_State) {
    let call_result = lua_pcall(state, 0, LUA_MULTRET, 0);
    if call_result != 0 {
        let mut size: usize = 0;
        println!(
            "failed to run file {}",
            CStr::from_ptr(lua_tolstring(state, -1, &mut size))
                .to_str()
                .unwrap()
        );
    }
}

unsafe fn load_string(state: *mut lua_State, to_load: &str) {
    let string = CString::new(to_load).unwrap();
    let result = luaL_loadstring(state, string.as_ptr());
    if result != 0 {
        let mut size: usize = 0;
        println!(
            "failed to load file {}",
            CStr::from_ptr(lua_tolstring(state, -1, &mut size))
                .to_str()
                .unwrap()
        );
    }
}

impl LanguageDriver for LuaRuntime {
    fn init(&mut self) {
        println!("{}", module_path!());
        let bindings_h = include_str!("../../bindings.h");
        let lua_ffi = include_str!("lua_ffi.lua");
        let regex_str = "^\\s*#\\s*include\\s+[<\"][^>\"]*[>\"].*$";
        let regex = RegexBuilder::new(regex_str)
            .multi_line(true)
            .build()
            .unwrap();
        let mut filtered_bindings = String::with_capacity(bindings_h.len());
        for capture in regex.split(bindings_h) {
            filtered_bindings.push_str(capture);
        }
        let combined_ffi = lua_ffi
            .replace("__ENGINE_MODULE_NAME__", "script_poc_core")
            .replace("__BINDINGS__", &filtered_bindings);

        unsafe {
            let state = luaL_newstate();
            luaL_openlibs(state);
            // assign the `engine` global
            {
                load_string(state, &combined_ffi);
                call_lua_func(state);
                lua_setfield(
                    state,
                    LUA_GLOBALSINDEX,
                    CString::new("engine").unwrap().as_ptr(),
                );
            }
            // assign the `ffi` global
            {
                load_string(state, "return require(\"ffi\")");
                call_lua_func(state);
                lua_setfield(
                    state,
                    LUA_GLOBALSINDEX,
                    CString::new("ffi").unwrap().as_ptr(),
                );
            }
            // load test script
            load_string(state, include_str!("test.lua"));
            self.run_func = luaL_ref(state, LUA_REGISTRYINDEX);
            self.state = state;
        }
    }

    fn run_stuff(&mut self) {
        unsafe {
            lua_rawgeti(self.state, LUA_REGISTRYINDEX, self.run_func);
            call_lua_func(self.state);
        }
    }

    fn unload(&mut self) {

    }
}
