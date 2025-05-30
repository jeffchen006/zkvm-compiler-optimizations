#![feature(c_variadic)]
#![allow(unused)]

use std::ffi::CString;

#[no_mangle]
pub extern "C" fn main_core() {
    unsafe {
        cmain();
    }
}

#[link(name = "spec-605", kind = "static")]
extern "C" {
    fn cmain() -> ();
}

static INPUT: &str = include_str!("inp_small.in");

#[no_mangle]
pub unsafe fn get_input() -> *const u8 {
    let cstr = CString::new(INPUT).unwrap();
    cstr.into_raw() as *const u8
}
