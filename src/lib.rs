#![no_std]
#![allow(internal_features)]
#![feature(ptr_internals)]

use crate::io::keyboard::{ControlKey, Key, Number, PrintableKey};
use crate::io::read_scancode;
use crate::memory::multiboot::{BootInformation, BootInformationHeader};
use crate::print::VGA_SCREEN_MANAGER;
use core::panic::PanicInfo;

mod data_structure;
mod io;
mod memory;
mod print;
mod shell;


#[no_mangle]
pub extern "C" fn kmain(multiboot_info_addr: u32) -> ! {
    VGA_SCREEN_MANAGER.lock().render_current_screen();
    let boot_info = unsafe {
        BootInformation::load(multiboot_info_addr as *const BootInformationHeader)
            .expect("Invalid boot information")
    };

    if let Some(mmap_tag) = boot_info.memory_map_tag() {
        println!("Physical memory areas:");
        for area in mmap_tag.memory_areas() {
            println!(
                "    start: 0x{:x}, length: 0x{:x}, type: {:?}",
                area.base_addr,
                area.length,
                area.region_type
            );
        }
    }


    let mut control_state = ControlKey::CtrlReleased;
    loop {
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
                Key::Navigation(nav) => VGA_SCREEN_MANAGER.lock().navigate(nav),
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
