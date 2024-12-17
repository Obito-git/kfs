use crate::data_structure::IteratorType::Capacity;
use crate::data_structure::StackVec;
use crate::io::color::{ColorCode, ScreenChar};
use crate::io::keyboard::NavigationKey;
use crate::io::{hide_cursor, set_cursor_position, show_cursor};
use crate::shell::Shell;
use core::fmt;
use core::fmt::Write;
use core::ptr::Unique;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;


const SHELLS_COUNT: usize = 3;
const VGA_BUFFER_ADDRESS: usize = 0xb8000;
pub const VGA_BUFFER_HEIGHT: usize = 25;
pub const SHELL_BUFFER_HEIGHT: usize = VGA_BUFFER_HEIGHT * 3;
pub const VGA_BUFFER_WIDTH: usize = 80;
pub const VGA_COMMAND_LINE_ROW_INDEX: usize = VGA_BUFFER_HEIGHT - 1;
pub const SHELL_COMMAND_LINE_ROW_INDEX: usize = SHELL_BUFFER_HEIGHT - 1;
pub const VGA_MIN_FIRST_LINE: usize = SHELL_BUFFER_HEIGHT - VGA_BUFFER_HEIGHT;

lazy_static! {
    static ref VGA_BUFFER: Mutex<Unique<VgaBuffer>> =
        Mutex::new(unsafe { Unique::new_unchecked(VGA_BUFFER_ADDRESS as *mut _) });
}

#[repr(transparent)]
pub struct VgaBuffer {
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

struct VgaScreen {
    current_buffer_first_line: usize,
    is_scrolling_mode_enabled: bool,
    shell: Shell,
    color: ColorCode,
}

impl VgaScreen {
    fn new(color: ColorCode) -> Self {
        Self {
            shell: Shell::new(),
            current_buffer_first_line: VGA_MIN_FIRST_LINE,
            is_scrolling_mode_enabled: false,
            color,
        }
    }

    fn get_data(&self) -> &[StackVec<u8, 80>] {
        self.shell.get_data().slice(
            self.current_buffer_first_line..self.current_buffer_first_line + VGA_BUFFER_HEIGHT,
        )
    }

    fn render_cursor(&mut self) {
        if !self.is_scrolling_mode_enabled {
            set_cursor_position(
                self.shell.cursor_position() as u8,
                VGA_COMMAND_LINE_ROW_INDEX as u8,
            );
        }
    }

    fn enable_scrolling(&mut self) {
        if !self.is_scrolling_mode_enabled {
            self.is_scrolling_mode_enabled = true;
            hide_cursor()
        }
    }

    fn disable_scrolling(&mut self) {
        if self.is_scrolling_mode_enabled {
            self.is_scrolling_mode_enabled = false;
            show_cursor()
        }
    }
}

pub struct VGAScreenManager {
    shells: [VgaScreen; SHELLS_COUNT],
    active_shell_index: usize,
}

impl VGAScreenManager {
    pub fn new(colors: &[ColorCode; SHELLS_COUNT]) -> Self {
        hide_cursor();
        show_cursor();
        Self {
            active_shell_index: 0,
            shells: [
                VgaScreen::new(colors[0]),
                VgaScreen::new(colors[1]),
                VgaScreen::new(colors[2]),
            ],
        }
    }

    fn current_screen(&mut self) -> &mut VgaScreen {
        &mut self.shells[self.active_shell_index]
    }

    pub fn render_current_screen(&mut self) {
        let color = self.current_screen().color;
        for (y, sub) in self.current_screen().get_data().iter().enumerate() {
            for (i, screen_char) in sub.iter(Capacity).enumerate() {
                unsafe {
                    VGA_BUFFER.lock().as_mut().chars[y][i].write(ScreenChar {
                        ascii_character: *screen_char,
                        color_code: color,
                    });
                }
            }
        }

        self.current_screen().render_cursor()
    }

    pub fn reset_scrolling_mode(&mut self) {
        if self.current_screen().is_scrolling_mode_enabled {
            self.current_screen().current_buffer_first_line = VGA_MIN_FIRST_LINE;
            self.current_screen().disable_scrolling();
        }
    }

    pub fn write_byte_to_the_command_line(&mut self, byte: u8) {
        self.reset_scrolling_mode();
        self.current_screen()
            .shell
            .write_byte_to_the_command_line(byte);
        self.render_current_screen();
    }

    pub fn navigate(&mut self, nav: NavigationKey) {
        match nav {
            NavigationKey::Down => {
                if self.current_screen().current_buffer_first_line < VGA_MIN_FIRST_LINE {
                    self.current_screen().current_buffer_first_line += 1;
                    if self.current_screen().current_buffer_first_line == VGA_MIN_FIRST_LINE {
                        self.current_screen().disable_scrolling();
                    }
                    self.render_current_screen();
                }
            }
            NavigationKey::Up => {
                if self.current_screen().current_buffer_first_line > 0 {
                    self.current_screen().current_buffer_first_line -= 1;
                    self.current_screen().enable_scrolling();
                    self.render_current_screen();
                }
            }
            _ => {
                self.current_screen().shell.move_cursor(nav);
                if self.current_screen().is_scrolling_mode_enabled {
                    self.reset_scrolling_mode();
                    self.render_current_screen()
                } else {
                    self.current_screen().render_cursor();
                }
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        let color = self.current_screen().color;
        if let Some((line_to_rerender, range)) = self.current_screen().shell.handle_backspace() {
            for i in range {
                unsafe {
                    VGA_BUFFER.lock().as_mut().chars[VGA_COMMAND_LINE_ROW_INDEX][i].write(
                        ScreenChar {
                            ascii_character: *line_to_rerender.get_unsafe(i),
                            color_code: color,
                        },
                    );
                }
            }
            self.current_screen().render_cursor()
        }
    }

    pub fn change_terminal(&mut self, terminal_nbr: usize) {
        self.active_shell_index = terminal_nbr;
        self.render_current_screen()
    }
}

impl Write for VGAScreenManager {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.current_screen().shell.write_str(s)?;
        self.render_current_screen();
        Ok(())
    }
}
