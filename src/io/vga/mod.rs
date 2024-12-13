use crate::data_structure::IteratorType::Capacity;
use crate::io::color::{ColorCode, ScreenChar};
use crate::io::keyboard::NavigationKey;
use crate::io::set_cursor_position;
use crate::println;
use crate::shell::Shell;
use core::fmt;
use core::fmt::Write;
use core::ptr::Unique;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

mod status_panel;

const SHELLS_COUNT: usize = 3;
const VGA_BUFFER_ADDRESS: usize = 0xb8000;
pub const VGA_BUFFER_HEIGHT: usize = 25;
pub const VGA_BUFFER_WIDTH: usize = 80;
pub const COMMAND_LINE_ROW_INDEX: usize = VGA_BUFFER_HEIGHT - 1;

lazy_static! {
    static ref VGA_BUFFER: Mutex<Unique<VgaBuffer>> =
        Mutex::new(unsafe { Unique::new_unchecked(VGA_BUFFER_ADDRESS as *mut _) });
}

#[repr(transparent)]
pub struct VgaBuffer {
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

pub struct VGAScreenManager {
    shells: [Shell; SHELLS_COUNT],
    colors: [ColorCode; SHELLS_COUNT],
    current_shell_index: usize,
}

impl VGAScreenManager {
    pub fn new(colors: &[ColorCode; SHELLS_COUNT]) -> Self {
        Self {
            current_shell_index: 0,
            colors: *colors,
            shells: [Shell::new(); SHELLS_COUNT],
        }
    }

    fn current_screen(&mut self) -> &mut Shell {
        &mut self.shells[self.current_shell_index]
    }

    pub fn render_current_screen(&mut self) {
        let color = self.colors[self.current_shell_index];
        for (y, sub) in self.current_screen().get_data().iter(Capacity).enumerate() {
            for (i, screen_char) in sub.iter(Capacity).enumerate() {
                unsafe {
                    VGA_BUFFER.lock().as_mut().chars[y][i].write(ScreenChar {
                        ascii_character: *screen_char,
                        color_code: color,
                    });
                }
            }
        }

        self.render_cursor()
    }

    fn render_cursor(&mut self) {
        set_cursor_position(
            self.current_screen().cursor_position() as u8,
            COMMAND_LINE_ROW_INDEX as u8,
        );
    }

    pub fn write_byte_to_the_command_line(&mut self, byte: u8) {
        self.current_screen().write_byte_to_the_command_line(byte);
        self.render_current_screen();
    }

    pub fn move_cursor(&mut self, nav: NavigationKey) {
        self.current_screen().move_cursor(nav);
        self.render_cursor();
    }

    pub fn handle_backspace(&mut self) {
        let color = self.colors[self.current_shell_index];
        if let Some((line_to_rerender, range)) = self.current_screen().handle_backspace() {
            for i in range {
                unsafe {
                    VGA_BUFFER.lock().as_mut().chars[COMMAND_LINE_ROW_INDEX][i].write(ScreenChar {
                        ascii_character: *line_to_rerender.get_unsafe(i),
                        color_code: color,
                    });
                }
            }
            self.render_cursor()
        }
    }

    pub fn change_terminal(&mut self, terminal_nbr: usize) {
        self.current_shell_index = terminal_nbr;
        self.render_current_screen()
    }

    #[allow(dead_code)]
    pub fn clear_screen() {
        for _ in 0..VGA_BUFFER_HEIGHT {
            println!();
        }
    }
}

impl Write for VGAScreenManager {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.current_screen().write_str(s)?;
        self.render_current_screen();
        Ok(())
    }
}
