#![windows_subsystem = "console"]

use enigo::*;
use std::{thread, time};

#[cfg(windows)]
extern crate winapi;

fn get_key_states(key: winapi::ctypes::c_int) -> (i16, i16) {
    let state: i16 = unsafe { winapi::um::winuser::GetKeyState(key) };
    let async_state: i16 = unsafe { winapi::um::winuser::GetAsyncKeyState(key) };
    (state, async_state)
}

fn handle_capslock(enigo: &mut Enigo, active: bool) -> bool {
    let mut is_active = active;
    let (state, async_state) = get_key_states(winapi::um::winuser::VK_CAPITAL);
    let has_pressed = (async_state & 1) > 0;
    if has_pressed {
        if (state & 1) > 0 {
            enigo.mouse_down(MouseButton::Left);
            is_active = true;
        } else {
            enigo.mouse_up(MouseButton::Left);
            is_active = false;
        }
    }
    is_active
}

fn handle_numlock(enigo: &mut Enigo, active: bool) -> bool {
    let mut is_active = active;
    let (state, async_state) = get_key_states(winapi::um::winuser::VK_NUMLOCK);
    let has_pressed = (async_state & 1) > 0;
    if has_pressed {
        is_active = (state & 1) > 0;
    }
    if is_active {
        enigo.mouse_click(MouseButton::Left);
    }
    is_active
}

fn main() {
    let mut enigo = Enigo::new();
    let mut cl_active: bool = false;
    let mut nl_active: bool = false;
    loop {
        cl_active = handle_capslock(&mut enigo, cl_active);
        if !cl_active {
            nl_active = handle_numlock(&mut enigo, nl_active);
        }
        if nl_active {
            thread::sleep(time::Duration::from_millis(10));
        } else {
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}
