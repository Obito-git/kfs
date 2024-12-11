use crate::io::color::{ColorCode, ScreenChar};
use crate::io::keyboard::NavigationKey;
use crate::io::vga::{LAST_VGA_ROW_INDEX, VGA_BUFFER_HEIGHT, VGA_BUFFER_WIDTH};
use crate::shell::SHELL_PROMPT;

pub struct VgaScreen {
    color_code: ColorCode,
    pub cursor_position: usize,
    chars_on_last_row: usize,
    buffer: [[ScreenChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

impl VgaScreen {
    pub fn new(color_code: ColorCode) -> Self {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code,
        };
        let mut screen = Self {
            color_code,
            cursor_position: 0,
            chars_on_last_row: 0,
            buffer: [[blank; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
        };
        for c in SHELL_PROMPT.chars() {
            screen.write_byte(c as u8);
        }
        screen
    }

    fn blank(&self) -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        }
    }

    //FIXME: allow automatic new line only for print macros
    pub fn write_byte(&mut self, byte: u8) {
        let byte = if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
            byte
        } else {
            0xfe
        };
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.chars_on_last_row < VGA_BUFFER_WIDTH {
                    let new_char = ScreenChar {
                        ascii_character: byte,
                        color_code: self.color_code,
                    };
                    if self.cursor_position < self.chars_on_last_row {
                        for i in (self.cursor_position..self.chars_on_last_row).rev() {
                            self.buffer[LAST_VGA_ROW_INDEX][i + 1] =
                                self.buffer[LAST_VGA_ROW_INDEX][i];
                        }
                    }
                    self.buffer[LAST_VGA_ROW_INDEX][self.cursor_position] = new_char;
                    self.cursor_position += 1;
                    self.chars_on_last_row += 1;
                }
            }
        }
    }

    pub fn handle_backspace(&mut self) -> Option<&[ScreenChar; VGA_BUFFER_WIDTH]> {
        if self.cursor_position > SHELL_PROMPT.len() {
            self.cursor_position -= 1;
            self.chars_on_last_row -= 1;
            for i in self.cursor_position..VGA_BUFFER_WIDTH - 1 {
                self.buffer[LAST_VGA_ROW_INDEX][i] = self.buffer[LAST_VGA_ROW_INDEX][i + 1]
            }
            self.buffer[LAST_VGA_ROW_INDEX][VGA_BUFFER_WIDTH - 1] = self.blank();
            Some(&self.buffer[LAST_VGA_ROW_INDEX])
        } else {
            None
        }
    }

    pub fn get_data(&self) -> &[[ScreenChar; VGA_BUFFER_WIDTH]] {
        &self.buffer
    }

    fn new_line(&mut self) {
        for row in 1..VGA_BUFFER_HEIGHT {
            for col in 0..VGA_BUFFER_WIDTH {
                let character = self.buffer[row][col];
                self.buffer[row - 1][col] = character;
            }
        }
        self.clear_row(VGA_BUFFER_HEIGHT - 1);
        self.cursor_position = 0;
        self.chars_on_last_row = 0;
        for c in SHELL_PROMPT.chars() {
            self.write_byte(c as u8);
        }
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..VGA_BUFFER_WIDTH {
            self.buffer[row][col] = self.blank();
        }
    }

    pub fn move_cursor(&mut self, nav: NavigationKey) {
        match nav {
            NavigationKey::Left => {
                if self.cursor_position > SHELL_PROMPT.len() {
                    self.cursor_position -= 1;
                }
            }
            NavigationKey::Right => {
                if self.cursor_position < self.chars_on_last_row {
                    self.cursor_position += 1;
                }
            }
            _ => (),
        }
    }
}
