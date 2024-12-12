#![no_std]
#![allow(internal_features)]
#![feature(ptr_internals)]

use crate::io::read_scancode;
use crate::print::VGA_SCREEN_MANAGER;
use core::panic::PanicInfo;
use io::keyboard::{ControlKey, Key, Number, PrintableKey};
use crate::data_structure::StackVec;

mod io;
mod print;
mod shell;
mod data_structure;


#[derive(Default, Copy, Clone)]
pub struct Foo();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    VGA_SCREEN_MANAGER.lock().render_current_screen();
    //println!("Hello, world!");

    loop {
        let mut control_state = ControlKey::CtrlReleased;
        if let Some(key) = read_scancode() {
            match key {
                Key::Printable(c) => match (control_state, c) {
                    (ControlKey::CtrlPressed, PrintableKey::Number(n)) => match n {
                        Number::N1 => VGA_SCREEN_MANAGER.lock().change_terminal(0),
                        Number::N2 => VGA_SCREEN_MANAGER.lock().change_terminal(1),
                        Number::N3 => VGA_SCREEN_MANAGER.lock().change_terminal(2),
                        _ => write_command_line_byte!(c.into()),
                    },
                    _ => write_command_line_byte!(c.into()),
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
    let hello = b"True Panic!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };
    //println!("Custom panic handler: {}", info);
    loop {}
}
