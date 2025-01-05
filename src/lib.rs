#![no_std]
#![allow(internal_features)]
#![feature(ptr_internals, abi_x86_interrupt)]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use crate::interrupts::enable_interrupts;
use crate::io::keyboard::{ControlKey, Key, Number, PrintableKey};
use crate::io::read_scancode;
use crate::memory::FRAME_ALLOCATOR;
use crate::memory::paging::{enable_paging, map_page, EntryFlags, VirtualAddress,};
use crate::print::VGA_SCREEN_MANAGER;

mod data_structure;
mod interrupts;
mod io;
mod memory;
mod panic;
mod print;
mod shell;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    VGA_SCREEN_MANAGER.lock().render_current_screen();

    let before_enabling_paging_var = 42_u32; // addr 0x2097c0

    /*

    unsafe {
        enable_paging();
        // before_enabling_paging_var is now accessible at 0xC02097C0
    }
    
    println!("After paging: {}", before_enabling_paging_var); //it still pointing at addr 0x2097c0 and Page Faults?
    
    let test_virtual_address = VirtualAddress::new(0xC000_0000); // Example virtual address
    let test_physical_frame = FRAME_ALLOCATOR.lock().allocate_frame().expect("Out of memory!");
    let test_physical_address = test_physical_frame.start_address() as u32;

    map_page(
        test_virtual_address,
        test_physical_address,
        EntryFlags::WRITABLE,
    );

    println!(
        "Mapped virtual address 0x{:x} to physical address 0x{:x}",
        test_virtual_address.as_u32(),
        test_physical_address,
    );

    unsafe {
        let ptr = test_virtual_address.as_u32() as *mut u32;
        *ptr = 0xDEADBEEF; // Write to the virtual address
        println!("Ptr value: 0x{:x}", *ptr);
        assert_eq!(*ptr, 0xDEADBEEF); // Verify the value
    }

    println!("Page mapping and access successful!");

    unsafe {
        enable_interrupts();
    }



    let mut vector: Vec<usize> = Vec::with_capacity(2024);
    let mut vector2 = vec![3,4];
    let mut vector3 = vec![5,6];

    
    println!("It didn't crash, {vector:?}, 0x{:x}", vector.as_ptr().addr());
    println!("It didn't crash, {vector2:?}, 0x{:x}", vector2.as_ptr().addr());
    println!("It didn't crash, {vector3:?}, 0x{:x}", vector3.as_ptr().addr());


     */
    /*
    unsafe {
        let dead_ptr = 0xdeadbeef as *mut u32;
        *dead_ptr = 0xDEADBEEF;
    }
     */



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
