type Handle = i64;
use std::sync::atomic::{AtomicUsize, Ordering};
#[repr(C)]
pub struct test_data {
    state: i32,
    dec: f32,
}
pub static test_var: AtomicUsize = AtomicUsize::new(0);
#[no_mangle]
pub extern "C" fn test(val: i32, data: test_data) -> f64 {
    // println!("called test with state {} and dec {}", data.state, data.dec);
    35.0
}

#[no_mangle]
pub extern "C" fn simple_test(val: i32, fl: i64) -> f64 {
    println!("called simple_test {} {} {}", val, fl, test_var.load(Ordering::Acquire));
    (val as i64 * fl * test_var.load(Ordering::Acquire) as i64) as f64
}

#[no_mangle]
pub extern "C" fn create_data() -> test_data {
    return test_data {
        state: 10,
        dec: 0.25,
    };
}

#[no_mangle]
pub extern "C" fn handle_test(handle: Handle) -> f64 {
    53.3
}

#[no_mangle]
pub extern "C" fn create_handle() -> Handle {
    return 53;
}
