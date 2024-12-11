use crate::io::color::{ColorCode, ScreenChar};
use crate::io::keyboard::NavigationKey;
use crate::io::set_cursor_position;
use crate::println;
use core::fmt;
use core::fmt::Write;
use core::ptr::Unique;
use lazy_static::lazy_static;
use spin::Mutex;
use vga_screen::VgaScreen;
use volatile::Volatile;

mod status_panel;
pub mod vga_screen;

const SCREENS_COUNT: usize = 3;
const VGA_BUFFER_ADDRESS: usize = 0xb8000;
pub const VGA_BUFFER_HEIGHT: usize = 25;
pub const VGA_BUFFER_WIDTH: usize = 80;
pub const LAST_VGA_ROW_INDEX: usize = VGA_BUFFER_HEIGHT - 1;

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
        set_cursor_position(
            self.current_screen().cursor_position as u8,
            LAST_VGA_ROW_INDEX as u8,
        );
    }

    pub fn move_cursor(&mut self, nav: NavigationKey) {
        self.current_screen().move_cursor(nav);
        self.render_cursor();
    }

    pub fn handle_backspace(&mut self) {
        if let Some(line_to_rerender) = self.current_screen().handle_backspace() {
            for (i, screen_char) in line_to_rerender.iter().enumerate() {
                unsafe {
                    VGA_BUFFER.lock().as_mut().chars[LAST_VGA_ROW_INDEX][i].write(*screen_char);
                }
            }
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
