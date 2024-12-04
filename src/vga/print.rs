use core::fmt;
use core::ptr::Unique;
use lazy_static::lazy_static;
use spin::mutex::Mutex;
use crate::vga::unit::{Color, ColorCode};
use crate::vga::{Writer, VGA_BUFFER_ADDRESS};

lazy_static! {
    static ref PRINT_WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe { Unique::new_unchecked(VGA_BUFFER_ADDRESS as *mut _) },
    });
}

#[doc(hidden)]
pub(crate) fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = PRINT_WRITER.lock();
    writer.write_fmt(args).unwrap();
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga::print::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
