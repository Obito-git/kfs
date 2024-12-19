mod screen;
pub mod manager;
pub mod color;

use color::ScreenChar;
use core::ptr::Unique;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;


const SHELLS_COUNT: usize = 3;
const VGA_BUFFER_ADDRESS: usize = 0xb8000;
pub const VGA_BUFFER_HEIGHT: usize = 25;
pub const SHELL_BUFFER_HEIGHT: usize = VGA_BUFFER_HEIGHT * 5;
pub const VGA_BUFFER_WIDTH: usize = 80;
pub const VGA_COMMAND_LINE_ROW_INDEX: usize = VGA_BUFFER_HEIGHT - 1;
pub const SHELL_COMMAND_LINE_ROW_INDEX: usize = SHELL_BUFFER_HEIGHT - 1;
pub const VGA_MIN_FIRST_LINE: usize = SHELL_BUFFER_HEIGHT - VGA_BUFFER_HEIGHT;

lazy_static! {
    static ref VGA_BUFFER: Mutex<Unique<VgaBuffer>> =
        Mutex::new(unsafe { Unique::new_unchecked(VGA_BUFFER_ADDRESS as *mut _) });
}

#[repr(transparent)]
struct VgaBuffer {
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}



