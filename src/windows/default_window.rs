/// DefaultWindow構造体

use windows::window::{Window, WindowFactory};

use std::mem::{zeroed};
use winapi::shared::windef::{HWND};
use winapi::um::winuser::{BeginPaint};
use winapi::um::wingdi::{TextOutA};

use std::rc::{Rc};

static SZ_TEXT: &'static [u8] = b"Hello, world!";

/// DefaultWindow構造体
pub struct DefaultWindow {
    hwnd: HWND,
    i: i32,
    j: i64
}

impl DefaultWindow {
    pub fn new(hwnd: HWND) -> DefaultWindow {
        DefaultWindow {hwnd: hwnd, i: 5, j: 6}
    }
}

impl Drop for DefaultWindow {
    fn drop(&mut self) -> () {
        println!("drop: {:?}", self.i);
    }
}

impl Window for DefaultWindow {
    fn on_create(&self) -> () {
        println!("on_create: {:?}", self.i);
    }
    fn on_destroy(&self) -> () {
        println!("on_destroy: {:?}", self.i);
    }
    fn paint(&self) -> () {
        println!("paint: {:?}", self.i);
        unsafe {
            let mut ps = zeroed();
            let hdc = BeginPaint(self.hwnd, &mut ps);
            TextOutA(hdc, 5, 5,
                     SZ_TEXT.as_ptr() as *const i8,
                     SZ_TEXT.len() as i32
            );
        }
    }
}

pub struct DefaultWindowFactory {}

impl DefaultWindowFactory {
    pub fn new () -> DefaultWindowFactory {
        DefaultWindowFactory {}
    }
}

impl WindowFactory for DefaultWindowFactory {
    fn create_window_object(&self, hwnd: HWND) -> Rc<Window> {
        Rc::new(DefaultWindow::new(hwnd))
    }
}