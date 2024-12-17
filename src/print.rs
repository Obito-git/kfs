use crate::io::vga::color::{Color, ColorCode};
use core::fmt;
use lazy_static::lazy_static;
use spin::mutex::Mutex;
use crate::io::vga::manager::VGAScreenManager;

const TERMINAL_STYLES: &[ColorCode; 3] = &[
    ColorCode::new(Color::LightGreen, Color::Black),
    ColorCode::new(Color::Yellow, Color::Black),
    ColorCode::new(Color::Pink, Color::Black),
];

lazy_static! {
    pub static ref VGA_SCREEN_MANAGER: Mutex<VGAScreenManager> =
        Mutex::new(VGAScreenManager::new(TERMINAL_STYLES));
}

#[doc(hidden)]
#[allow(dead_code)]
pub fn _hello_world() {
    static HELLO: &[u8] = b"Hello World!";

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}


#[doc(hidden)]
pub fn _write_to_command_line(byte: u8) {
    let mut screen_mgr = VGA_SCREEN_MANAGER.lock();
    screen_mgr.write_byte_to_the_command_line(byte)
}

#[macro_export]
macro_rules! write_command_line_byte {
    ($byte:expr) => ({
        $crate::print::_write_to_command_line($byte);
    });
}

#[doc(hidden)]
pub(crate) fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut screen_mgr = VGA_SCREEN_MANAGER.lock();
    screen_mgr.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::print::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
