use std::ffi::CStr;
use std::mem::transmute;
use std::os::raw::{c_char, c_int};

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = [71u8, 80, 76, 0]; //b"GPL\0"

#[no_mangle]
#[link_section = "version"]
pub static _version: u32 = 0xFFFFFFFE;

#[no_mangle]
#[link_section = "kprobe/SyS_clone"]
pub extern "C" fn kprobe__sys_clone(ctx: *mut u8) -> c_int {
    let BPF_FUNC_trace_printk = unsafe { transmute::<i64, fn(*const c_char, c_int) -> c_int>(6) };

    let msg: [c_char; 17] = [
        104, 101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 114, 117, 115, 116, 10, 0,
    ]; //b"hello from Rust\0"
    BPF_FUNC_trace_printk((&msg).as_ptr(), 17);
    return 0;
}
