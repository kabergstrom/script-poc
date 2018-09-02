extern crate script_poc_core;
#[no_mangle]
pub unsafe extern "C" fn run() {
    for i in 0..1000{
    script_poc_core::ext_api::simple_test(i, 35);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
