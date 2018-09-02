#![feature(libc)]
use std::io::{self, BufRead};
extern crate libc;
extern crate lua_jit_sys;
extern crate regex;
pub mod ext_api;
pub mod lua_driver;
pub mod rust_driver;
use std::fmt::Display;

pub trait LanguageDriver {
    fn init(&mut self);
    fn run_stuff(&mut self);
    fn unload(&mut self);
}
pub extern "C" fn run() {
    ext_api::test_var.store(58, std::sync::atomic::Ordering::Relaxed);
    let mut runtime = lua_driver::new_lua_runtime();
    runtime.init();
    runtime.run_stuff();

    let mut rust = rust_driver::new_rust_runtime();
    rust.init();
    rust.run_stuff();
    rust.unload();

    let stdin = io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();
    rust.init();
    rust.run_stuff();
    rust.unload();
}

pub extern "C" fn test_run(driver: &mut Box<LanguageDriver>) {
    driver.run_stuff();
}


pub struct a_rust_struct {
    pub num: i32,
    pub float: f32, 
}

#[no_mangle]
pub extern fn test_woot(val: a_rust_struct) {
    println!("test rust stuff {}", val.num);
}
