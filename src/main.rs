#![no_main]
#![feature(link_args)]

extern crate winapi;

mod windows;

use winapi::shared::minwindef::{HINSTANCE};
use winapi::shared::ntdef::{LPCSTR};
use winapi::ctypes::{c_int};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMain(h_instance: HINSTANCE,
                               h_prev_instance: HINSTANCE,
                               cmd_line: LPCSTR,
                               cmd_show: c_int) -> c_int {
    windows::window::win_main(h_instance, h_prev_instance, cmd_line, cmd_show)
}
