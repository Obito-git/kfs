pub mod keyboard;
pub mod vga;

use crate::io::keyboard::Key;
use crate::io::vga::VGA_BUFFER_WIDTH;
use core::arch::asm;

const KEYBOARD_STATUS_PORT: u16 = 0x64;
const KEYBOARD_DATA_PORT: u16 = 0x60;

/*
   TODO: move to another module, or somehow categorize it
   TODO: inline assembly should be arch dependent
*/

pub fn set_cursor_position(x: u8, y: u8) {
    let position: u16 = y as u16 * VGA_BUFFER_WIDTH as u16 + x as u16;
    outb(0x3D4, 0x0F);
    outb(0x3D5, (position & 0xFF) as u8);
    outb(0x3D4, 0x0E);
    outb(0x3D5, ((position >> 8) & 0xFF) as u8);
}

pub fn hide_cursor() {
    outb(0x3D4, 0x0A);
    outb(0x3D5, 0x20);
}

pub fn show_cursor() {
    outb(0x3D4, 0x0A);
    outb(0x3D5, 0x00);
}

fn inb(port: u16) -> u8 {
    unsafe {
        let ret: u8;
        asm!("in al, dx", out("al") ret, in("dx") port);
        ret
    }
}

pub fn get_esp() -> u32 {
    unsafe {
        let esp: u32;
        asm!("mov {}, esp", out(reg) esp);
        esp
    }
}

fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("al") value, in("dx") port);
    }
}

fn outw(port: u16, value: u16) {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") value);
    }
}

pub fn exit_qemu() {
    outw(0x604, 0x2000);
}

/// via keyboard controller
pub fn reboot() {
    outb(0x64, 0xFE);
}

pub fn read_scancode() -> Option<Key> {
    if inb(KEYBOARD_STATUS_PORT) & 1 != 0 {
        let scancode = inb(KEYBOARD_DATA_PORT);
        Key::from_scancode(scancode)
    } else {
        None
    }
}
