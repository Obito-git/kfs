use crate::io::color::{Color, ColorCode};
use crate::io::vga::VGAScreenManager;
use core::fmt;
use lazy_static::lazy_static;
use spin::mutex::Mutex;

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
