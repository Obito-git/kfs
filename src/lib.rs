#![no_std]
#![allow(internal_features)]
#![feature(ptr_internals)]

use crate::io::read_scancode;
use crate::print::VGA_SCREEN_MANAGER;
use core::panic::PanicInfo;
use io::keyboard::{ControlKey, Key, Number, PrintableKey};

mod io;
mod print;
mod shell;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    VGA_SCREEN_MANAGER.lock().render_current_screen();
    let mut control_state = ControlKey::CtrlReleased;
    loop {
        if let Some(key) = read_scancode() {
            match key {
                Key::Printable(c) => match (control_state, c) {
                    (ControlKey::CtrlPressed, PrintableKey::Number(n)) => match n {
                        Number::N1 => VGA_SCREEN_MANAGER.lock().change_terminal(0),
                        Number::N2 => VGA_SCREEN_MANAGER.lock().change_terminal(1),
                        Number::N3 => VGA_SCREEN_MANAGER.lock().change_terminal(2),
                        _ => print!("{}", c.to_char()),
                    },
                    _ => print!("{}", c.to_char()),
                },
                Key::Navigation(nav) => VGA_SCREEN_MANAGER.lock().move_cursor(nav),
                Key::Control(ctr) => match ctr {
                    ControlKey::CtrlPressed | ControlKey::CtrlReleased => control_state = ctr,
                    ControlKey::Backspace => VGA_SCREEN_MANAGER.lock().handle_backspace(),
                },
            }
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Custom panic handler: {}", info);
    loop {}
}
