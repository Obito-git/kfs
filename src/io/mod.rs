pub mod color;
pub mod keyboard;
pub mod vga;

use crate::io::vga::VGA_BUFFER_WIDTH;
use core::arch::asm;
use keyboard::Key;

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

#[derive(Debug, PartialEq)]
pub enum CpuMode {
    Real,
    Protected,
    Long,
    Unknown,
}

impl CpuMode {
    pub fn to_str(&self) -> &str {
        match self {
            CpuMode::Real => "CPU is in Real Mode",
            CpuMode::Protected => "CPU is in Protected Mode",
            CpuMode::Long => "CPU is in Long Mode",
            CpuMode::Unknown => "CPU is in Unknown Mode",
        }
    }
}

pub fn get_cpu_mode() -> CpuMode {
    // Check CR0 PE bit for protected mode
    let cr0: u32; // Changed from u64 to u32 for i386
    unsafe {
        asm!(
        "mov {}, cr0",
        out(reg) cr0,
        options(nomem, nostack, preserves_flags)
        );
    }

    let efer_lo: u32;
    let _efer_hi: u32;
    unsafe {
        asm!(
        "mov ecx, 0xC0000080",
        "rdmsr",
        out("eax") efer_lo,
        out("edx") _efer_hi,
        options(nomem, nostack, preserves_flags)
        );
    }

    let pe_bit = cr0 & 1;
    let lma_bit = ((efer_lo) >> 8) & 1;

    match (pe_bit, lma_bit) {
        (0, 0) => CpuMode::Real,
        (1, 0) => CpuMode::Protected,
        (1, 1) => CpuMode::Long,
        _ => CpuMode::Unknown,
    }
}
