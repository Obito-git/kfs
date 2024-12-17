use crate::data_structure::IteratorType::Capacity;
use crate::io::vga::color::{ColorCode, ScreenChar};
use crate::io::vga::screen::VgaScreen;
use crate::io::vga::{SHELLS_COUNT, VGA_BUFFER, VGA_COMMAND_LINE_ROW_INDEX, VGA_MIN_FIRST_LINE};
use crate::io::{hide_cursor, show_cursor};
use core::fmt;
use core::fmt::Write;
use crate::io::keyboard::NavigationKey;

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
