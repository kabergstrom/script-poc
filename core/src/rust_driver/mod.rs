extern crate sharedlib;

use LanguageDriver;
#[derive(Default)]
pub struct RustRuntime {
    lib: Option<sharedlib::LibUnsafe>,
    func: Option<sharedlib::FuncUnsafe<unsafe extern "C" fn() -> ()>>,
}
pub fn new_rust_runtime() -> Box<LanguageDriver> {
    return Box::new(RustRuntime {
        ..Default::default()
    });
}

impl LanguageDriver for RustRuntime {
    fn init(&mut self) {
        unsafe {
            println!("{}", ::std::env::current_dir().unwrap().to_string_lossy());
            let lib = sharedlib::LibUnsafe::new(format!("{}/{}", ::std::env::current_dir().unwrap().to_string_lossy(), "rust_plugin/target/debug/rust_plugin.dll")).unwrap();
            self.func = Some(lib.find_func("run").unwrap());
            self.lib = Some(lib);
        }
    }

    fn run_stuff(&mut self) {
        unsafe {
            (self.func.unwrap())();
        }
    }

    fn unload(&mut self) {
        *self = RustRuntime { ..Default::default() };
    }
}
