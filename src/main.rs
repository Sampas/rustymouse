use enigo::*;
use std::{thread, time};

#[cfg(windows)]
extern crate winapi;

fn main() {
    let mut enigo = Enigo::new();
    loop {
        let state: i16 =
            unsafe { winapi::um::winuser::GetKeyState(winapi::um::winuser::VK_CAPITAL) };
        let async_state: i16 =
            unsafe { winapi::um::winuser::GetAsyncKeyState(winapi::um::winuser::VK_CAPITAL) };
        let has_pressed = (async_state & 1) > 0;
        if has_pressed {
            if (state & 1) > 0 {
                enigo.mouse_down(MouseButton::Left);
            } else {
                enigo.mouse_up(MouseButton::Left);
            }
        }
        thread::sleep(time::Duration::from_millis(100));
    }
}
