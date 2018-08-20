/// Windowトレイトと関連ユーティリティ

use std::mem::{size_of, zeroed, transmute};
use winapi::shared::minwindef::{HINSTANCE, ATOM, UINT, WPARAM, LPARAM, LRESULT, LPVOID};
use winapi::shared::ntdef::{LONG, LPCSTR, LPCWSTR};
use winapi::ctypes::{c_int};
use winapi::shared::windef::{HWND, HMENU, HICON, HCURSOR, HBRUSH};
use winapi::um::winuser::{WNDCLASSEXW, CS_VREDRAW, CS_HREDRAW, COLOR_WINDOWFRAME, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT};
use winapi::um::winuser::{WM_CREATE, WM_DESTROY, WM_PAINT};
use winapi::um::winuser::{RegisterClassExW, CreateWindowExW, ShowWindow, MessageBoxA};
use winapi::um::winuser::{GetMessageW, TranslateMessage, DispatchMessageW};
use winapi::um::winuser::{DefWindowProcW, PostQuitMessage};
use winapi::um::winuser::{SetWindowLongW, GetWindowLongW};

use std::rc::{Rc};

static SZ_CLASS: &'static [u8] = b"c\0l\0a\0s\0s\0\0\0";
static SZ_TITLE: &'static [u8] = b"t\0i\0t\0l\0e\0\0\0";

/// Windowトレイト
pub trait Window {
    /// WM_CREATEではない
    fn on_create(&self) -> () {
        println!("on_create:");
    }
    /// WM_DESTROY
    fn on_destroy(&self) -> () {
        println!("on_destory:");
    }
    /// WM_PAINT
    fn paint(&self) -> () {
        println!("paint:");
    }
}

pub trait WindowFactory {
    fn create_window_object(&self, hwnd: HWND) -> Rc<Window>;
}

/// WinMainより呼ばれる
pub fn win_main<T: WindowFactory>(h_instance: HINSTANCE,
                _h_prev_instance: HINSTANCE,
                _cmd_line: LPCSTR,
                cmd_show: c_int,
                factory: T) -> c_int {
    unsafe {
        match register_window_class(h_instance) {
            0 => {
                MessageBoxA(
                    0 as HWND,
                    b"Call to RegisterClassEx failed!\0".as_ptr() as *const i8,
                    b"Win32 Guided Tour\0".as_ptr() as *const i8,
                    0 as UINT
                );
            },
            _atom => {
                let hwnd = create_window(h_instance);
                if hwnd.is_null() {
                    MessageBoxA(
                        0 as HWND,
                        b"Call to CreateWindow failed!\0".as_ptr() as *const i8,
                        b"Win32 Guided Tour\0".as_ptr() as *const i8,
                        0 as UINT
                    );
                } else {
                    init_window(hwnd, cmd_show);
                    {
                        // ウインドウ属性にWindowオブジェクトを設定する
                        let window = factory.create_window_object(hwnd);
                        {
                            // WM_CREATE発生後なのでここで呼ぶ
                            window.on_create();
                        }
                        set_window(hwnd, window);
                    }
                    event_loop();
                };
            }
        };
        0
    }
}

/// ウインドウクラスを登録する
///
/// # Arguments
///
/// * `h_instance` -
///
/// # Return value
///
///
fn register_window_class(h_instance: HINSTANCE) -> ATOM {
    let size = size_of::<&Window>() as i32;
    println!("size = {:?}", size);
    unsafe {
        let wcex = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: CS_VREDRAW | CS_HREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            // Window trait objectを格納
            cbWndExtra: size,
            hInstance: h_instance,
            hIcon: 0 as HICON,
            hCursor: 0 as HCURSOR,
            hbrBackground: (COLOR_WINDOWFRAME) as HBRUSH,
            lpszMenuName: 0 as LPCWSTR,
            lpszClassName: SZ_CLASS.as_ptr() as *const u16,
            hIconSm: 0 as HICON,
        };
        RegisterClassExW(&wcex)
    }
}

/// ウインドウを作成する
fn create_window(h_instance: HINSTANCE) -> HWND {
    unsafe {
        CreateWindowExW(
            0,
            SZ_CLASS.as_ptr() as *const u16,
            SZ_TITLE.as_ptr() as *const u16,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT, CW_USEDEFAULT, 500, 100,
            0 as HWND, 0 as HMENU,
            h_instance,
            9999 as LPVOID
        )
    }
}

/// ウインドウを初期化する
fn init_window(hwnd: HWND, cmd_show: c_int) -> () {
    unsafe {
        ShowWindow(hwnd, cmd_show);
    }
}

/// イベントループ
fn event_loop() -> () {
    unsafe {
        let mut msg = zeroed();
        while GetMessageW(&mut msg, 0 as HWND, 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

/// イベントに従ってWindowオブジェクトのメソッドを呼び出す
unsafe extern "system"
fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
//            // lpCreateParamsにCreateWindowExのlpParamが入っている
//            let cs = lparam as *const CREATESTRUCTW;
//            let p = (*cs).lpCreateParams as i32;
            // NOTE: まだウインドウ属性が入っていないのでWindow.on_createは呼び出せない
            0
        },
        WM_DESTROY => {
            let window = get_window(hwnd);
            {
                window.on_destroy();
            }
            PostQuitMessage(0);
            // NOTE: release_windowを呼ばないことでオブジェクトを破棄する
            0
        },
        WM_PAINT => {
            let window = get_window(hwnd);
            {
                window.paint();
            }
            release_window(window);
            0
        },
        _ => {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }
}

/// ウインドウ属性にWindowオブジェクトを登録する
///
///  # Arguments
///
/// * `hwnd` - ウインドウハンドル
/// * `window` - 登録するWindowオブジェクト
///
///  # Return value
///
fn set_window(hwnd: HWND, window: Rc<Window>) -> () {
    unsafe {
        let raw = Rc::into_raw(window);
        let p = transmute::<*const Window, u128>(raw);
        let p1 = ((p >> 96) & 0xffffffff) as u32;
        let p2 = ((p >> 64) & 0xffffffff) as u32;
        let p3 = ((p >> 32) & 0xffffffff) as u32;
        let p4 = ((p >> 0) & 0xffffffff) as u32;
        SetWindowLongW(hwnd, 0, transmute::<u32, LONG>(p1));
        SetWindowLongW(hwnd, 4, transmute::<u32, LONG>(p2));
        SetWindowLongW(hwnd, 8, transmute::<u32, LONG>(p3));
        SetWindowLongW(hwnd, 12, transmute::<u32, LONG>(p4));
    }
}

/// ウインドウ属性からWindowオブジェクトを取得する
///
///  # Arguments
///
/// * `hwnd` - ウインドウハンドル
///
///  # Return value
///
/// 取得したWindowオブジェクト
fn get_window(hwnd: HWND) -> Rc<Window> {
    unsafe {
        let p1 = transmute::<LONG, u32>(GetWindowLongW(hwnd, 0));
        let p2 = transmute::<LONG, u32>(GetWindowLongW(hwnd, 4));
        let p3 = transmute::<LONG, u32>(GetWindowLongW(hwnd, 8));
        let p4 = transmute::<LONG, u32>(GetWindowLongW(hwnd, 12));
        let p = (((p1 as u128) & 0xffffffff) << 96) +
            (((p2 as u128) & 0xffffffff) << 64) +
            (((p3 as u128) & 0xffffffff) << 32) +
            (((p4 as u128) & 0xffffffff) << 0);
        Rc::from_raw(transmute::<u128, *const Window>(p))
    }
}

/// ウインドウをRcの管理下から外す(メモリを解放しない)
///
/// # Arguments
///
/// * window - Windowオブジェクト
///
fn release_window(window: Rc<Window>) -> () {
    Rc::into_raw(window);
}
