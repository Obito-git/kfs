use crate::color::{ColorCode, ScreenChar};
use crate::cpu_io::set_cursor_position;
use crate::keyboard::NavigationKey;
use crate::println;
use crate::vga_screen::VgaScreen;
use core::fmt;
use core::fmt::Write;
use core::ptr::Unique;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const SCREENS_COUNT: usize = 3;
const VGA_BUFFER_ADDRESS: usize = 0xb8000;
pub const VGA_BUFFER_HEIGHT: usize = 25;
pub const VGA_BUFFER_WIDTH: usize = 80;

lazy_static! {
    static ref VGA_BUFFER: Mutex<Unique<VgaBuffer>> =
        Mutex::new(unsafe { Unique::new_unchecked(VGA_BUFFER_ADDRESS as *mut _) });
}

#[repr(transparent)]
pub struct VgaBuffer {
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

pub struct VGAScreenManager {
    writers: [VgaScreen; SCREENS_COUNT],
    current_writer_index: usize,
}

impl VGAScreenManager {
    pub fn new(colors: &[ColorCode; SCREENS_COUNT]) -> Self {
        Self {
            current_writer_index: 0,
            writers: [
                VgaScreen::new(colors[0]),
                VgaScreen::new(colors[1]),
                VgaScreen::new(colors[2]),
            ],
        }
    }

    fn current_screen(&mut self) -> &mut VgaScreen {
        &mut self.writers[self.current_writer_index]
    }

    pub fn render_current_screen(&mut self) {
        for (y, sub) in self.current_screen().get_data().iter().enumerate() {
            for (i, screen_char) in sub.iter().enumerate() {
                unsafe {
                    VGA_BUFFER.lock().as_mut().chars[y][i].write(*screen_char);
                }
            }
        }
        self.render_cursor()
    }

    fn render_cursor(&mut self) {
        let position = &self.current_screen().get_cursor();
        set_cursor_position(position.x as u8, position.y as u8);
    }

    pub fn move_cursor(&mut self, nav: NavigationKey) {
        if self.current_screen().move_cursor(nav) {
            self.render_current_screen();
        } else {
            self.render_cursor()
        }
    }

    pub fn change_terminal(&mut self, terminal_nbr: usize) {
        self.current_writer_index = terminal_nbr;
        self.render_current_screen()
    }

    #[allow(dead_code)]
    pub fn clear_screen() {
        for _ in 0..VGA_BUFFER_HEIGHT {
            println!();
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.current_screen().write_byte(byte);
        self.render_current_screen()
    }
}

impl Write for VGAScreenManager {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}
