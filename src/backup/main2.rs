/*
 * Windows File open dialog demo.
 * rustc 1.22.1 (05e2e1c41 2017-11-22): stable.
 * This file must be saved as UTF-8 format.
 *
 * 2018/01/02 by audin
 */
#![windows_subsystem = "windows"]

extern crate winapi;
extern crate user32;

use std::env;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::mem::size_of;
use winapi::um::commdlg::{OPENFILENAMEW, OFN_HIDEREADONLY};
use winapi::um::commdlg::GetOpenFileNameW;
use std::result;

const SZW_SIZE: usize = 512;
// Type alias to simplify representation.
type Result<T> = result::Result<T,String>;
type TSzwBuf   = [u16; SZW_SIZE];

// Open the fileopen dialog with Windows GUI.
#[allow(non_snake_case)]
fn file_open_dialog() -> Result<String> {
    let szwFilter = str_to_szw("プログラム:(*.exe)\0*.exe\0すべて:(*)\0*\0");
    let szwTitle  = str_to_szw("ファイルを選択します");
    let mut szw_buf: TSzwBuf = [0; SZW_SIZE];
    let mut dlgOpen = OPENFILENAMEW {
        lStructSize:       size_of::<OPENFILENAMEW>() as u32, //DWORD,
        hwndOwner:         null_mut(),                        // HWND,
        hInstance:         null_mut(),                        // HINSTANCE,
        lpstrFilter:       szwFilter.as_ptr(),                // LPCWSTR,
        lpstrCustomFilter: null_mut(),                        // LPWSTR,
        nMaxCustFilter:    0,                                 // DWORD,
        nFilterIndex:      0,                                 // DWORD,
        lpstrFile:         szw_buf.as_mut_ptr(),              // LPWSTR,
        nMaxFile:          szw_buf.len() as u32,              // DWORD,
        lpstrFileTitle:    null_mut(),                        // LPWSTR,
        nMaxFileTitle:     0,                                 // DWORD,
        lpstrInitialDir:   null_mut(),                        // LPCWSTR,
        lpstrTitle:        szwTitle.as_ptr(),                 // LPCWSTR,
        Flags:             OFN_HIDEREADONLY,                  // DWORD,
        nFileOffset:       0,                                 // WORD,
        nFileExtension:    0,                                 // WORD,
        lpstrDefExt:       null_mut(),                        // LPCWSTR,
        lCustData:         0,                                 // LPARAM,
        lpfnHook:          None,                              // LPOFNHOOKPROC,
        lpTemplateName:    null_mut(),                        // LPCWSTR,
        pvReserved:        null_mut(),                        // *mut c_void,
        dwReserved:        0,                                 // DWORD,
        FlagsEx:           0,                                 // DWORD,
    };

    match unsafe { GetOpenFileNameW(&mut dlgOpen) } {
        0 => Err("Nothing is selected !".to_string()),
        _ => szw_to_string( &szw_buf ),
    }
}

// Convert from TSzwBuf([u16;512]) to String type.
fn szw_to_string( szwbuf: &TSzwBuf ) -> Result<String> {
    szwbuf.iter()
        .position(|wch| wch == &0)
        .ok_or("String : Can't find zero terminator !".to_owned())
        .and_then(|ix| String::from_utf16( &szwbuf[..ix] )
            .map_err(|e| e.to_string()))
}

// Convert from String to Vec<u16> with trailing \0.
fn str_to_szw(str_body: &str) -> Vec<u16> {
    return OsStr::new(str_body)
        .encode_wide()
        .chain(once(0))  // 終端文字\0を追加
        .collect::<Vec<u16>>();
}

fn main() {

    let args = env::args().collect::<Vec<String>>();

    if args.len() >= 2 {
        println!("{}", &args[1]);
    }
        else {
            match file_open_dialog() {
                Ok(file_path) => println!("{}", file_path),
                Err(e)        => println!("{}", e),
            };
        }
}