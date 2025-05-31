#![allow(unused)]

use std::ffi::CString;

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

#[link(name = "spec-631", kind = "static")]
extern "C" {
    fn cmain() -> ();
}

static mut INPUT: String = String::new();

#[no_mangle]
pub unsafe fn read_string() -> *const u8 {
    unsafe {
        #[allow(static_mut_refs)]
        let cstr = CString::new(INPUT.clone()).unwrap();
        cstr.into_raw() as *const u8
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn main_core(input: String) {
    unsafe {
        INPUT = input;
        cmain();
    }
}
