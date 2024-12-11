pub mod color;
pub mod keyboard;
pub mod vga;

use crate::io::vga::VGA_BUFFER_WIDTH;
use core::arch::asm;
use keyboard::Key;

const KEYBOARD_STATUS_PORT: u16 = 0x64;
const KEYBOARD_DATA_PORT: u16 = 0x60;

pub fn set_cursor_position(x: u8, y: u8) {
    let position: u16 = y as u16 * VGA_BUFFER_WIDTH as u16 + x as u16;
    outb(0x3D4, 0x0F);
    outb(0x3D5, (position & 0xFF) as u8);
    outb(0x3D4, 0x0E);
    outb(0x3D5, ((position >> 8) & 0xFF) as u8);
}

fn inb(port: u16) -> u8 {
    unsafe {
        let ret: u8;
        asm!("in al, dx", out("al") ret, in("dx") port);
        ret
    }
}

pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("al") value, in("dx") port);
    }
}

pub fn read_scancode() -> Option<Key> {
    if inb(KEYBOARD_STATUS_PORT) & 1 != 0 {
        let scancode = inb(KEYBOARD_DATA_PORT);
        Key::from_scancode(scancode)
    } else {
        None
    }
}
