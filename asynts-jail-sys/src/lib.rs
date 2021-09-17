extern crate libc;

extern "C" {
    fn child_main_impl(argument: *mut libc::c_void) -> libc::c_int;
}

pub extern "C" fn child_main(argument: *mut libc::c_void) -> libc::c_int {
    unsafe { child_main_impl(argument) }
}
