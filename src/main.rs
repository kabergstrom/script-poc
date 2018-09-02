#![feature(test)]
extern crate script_poc_core;
extern crate test;
use test::Bencher;

fn main() {
    println!("Hello, world!");
    script_poc_core::run();
}

#[bench]
fn bench_lua_driver_run(b: &mut Bencher) {
    let mut runtime = script_poc_core::lua_driver::new_lua_runtime();
    runtime.init();
    runtime.run_stuff();
    b.iter(|| {
        runtime.run_stuff();
    });
}


#[bench]
fn bench_rust_driver(b: &mut Bencher) {
    let mut runtime = script_poc_core::rust_driver::new_rust_runtime();
    runtime.init();
    runtime.run_stuff();
    b.iter(|| {
        runtime.run_stuff();
    });
}