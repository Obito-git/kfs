use crate::color::{ColorCode, ScreenChar};
use crate::cursor::Cursor;
use crate::keyboard::NavigationKey;
use crate::vga_screen_manager::{VGA_BUFFER_HEIGHT, VGA_BUFFER_WIDTH};

const HEADER: &'static str = r"
____________/\\\_______/\\\\\\\\\_____
 __________/\\\\\_____/\\\///////\\\___
  ________/\\\/\\\____\///______\//\\\__
   ______/\\\/\/\\\______________/\\\/___
    ____/\\\/__\/\\\___________/\\\//_____
     __/\\\\\\\\\\\\\\\\_____/\\\//________
      _\///////////\\\//____/\\\/___________
       ___________\/\\\_____/\\\\\\\\\\\\\\\_
        ___________\///_____\///////////////__
";

pub struct VgaScreen {
    color_code: ColorCode,
    cursor: Cursor,
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
            cursor: Cursor {
                x: 0,
                y: VGA_BUFFER_HEIGHT - 1,
            },
            buffer: [[blank; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
        };
        for c in HEADER.chars() {
            screen.write_byte(c as u8)
        }
        screen
    }

    fn blank(&self) -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        let byte = if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
            byte
        } else {
            0xfe
        };
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.cursor.x >= VGA_BUFFER_WIDTH {
                    self.new_line();
                }

                let cursor = self.cursor;
                let color_code = self.color_code;
                self.buffer[cursor.y][cursor.x] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.cursor.x += 1;
            }
        }
    }

    pub fn get_cursor(&self) -> Cursor {
        Cursor {
            x: self.cursor.x,
            y: self.cursor.y,
        }
    }

    pub fn handle_backspace(&mut self) -> Option<&[ScreenChar; VGA_BUFFER_WIDTH]> {
        if self.cursor.x > 0 {
            let current_y = self.cursor.y;
            self.cursor.x -= 1;
            for i in self.cursor.x..VGA_BUFFER_WIDTH - 1 {
                self.buffer[current_y][i] = self.buffer[current_y][i + 1]
            }
            self.buffer[current_y][VGA_BUFFER_WIDTH - 1] = self.blank();
            Some(&self.buffer[current_y])
        } else {
            None
        }
    }

    pub fn get_data(&self) -> &[[ScreenChar; VGA_BUFFER_WIDTH]] {
        let start = if self.cursor.y >= VGA_BUFFER_HEIGHT {
            VGA_BUFFER_HEIGHT
        } else {
            0
        };
        &self.buffer[start..start + VGA_BUFFER_HEIGHT]
    }

    fn new_line(&mut self) {
        for row in 1..VGA_BUFFER_HEIGHT {
            for col in 0..VGA_BUFFER_WIDTH {
                let character = self.buffer[row][col];
                self.buffer[row - 1][col] = character;
            }
        }
        self.clear_row(VGA_BUFFER_HEIGHT - 1);
        self.cursor.x = 0;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..VGA_BUFFER_WIDTH {
            self.buffer[row][col] = self.blank();
        }
    }

    pub fn move_cursor(&mut self, nav: NavigationKey) {
        match nav {
            NavigationKey::Left => self.cursor.move_left(),
            NavigationKey::Right => self.cursor.move_right(&VGA_BUFFER_WIDTH),
            _ => (),
        }
    }
}
