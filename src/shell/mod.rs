use crate::data_structure::{IteratorType, StackVec};
use crate::io::keyboard::NavigationKey;
use crate::io::vga::{COMMAND_LINE_ROW_INDEX, VGA_BUFFER_HEIGHT, VGA_BUFFER_WIDTH};
use core::fmt;
use core::fmt::Write;

pub const SHELL_PROMPT: &str = "$> ";
pub const MAX_COMMAND_LEN: usize = VGA_BUFFER_WIDTH - SHELL_PROMPT.len();
const MAX_COMMANDS_IN_HISTORY: usize = 20;

struct CommandExecutor {}

enum Command {
    Reboot,
    Help,
}
const COMMANDS: &[([u8; MAX_COMMAND_LEN], Command)] = &[([2; MAX_COMMAND_LEN], Command::Reboot)];
impl Command {}

impl TryFrom<&[u8; MAX_COMMAND_LEN]> for Command {
    type Error = &'static str;

    fn try_from(value: &[u8; MAX_COMMAND_LEN]) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Copy)]
pub struct Shell {
    cursor_position: usize,
    buffer: StackVec<StackVec<u8, VGA_BUFFER_WIDTH>, VGA_BUFFER_HEIGHT>,
    current_command: (usize, [u8; MAX_COMMAND_LEN]),
}

impl Shell {
    pub fn new() -> Self {
        let buffer: StackVec<StackVec<u8, VGA_BUFFER_WIDTH>, VGA_BUFFER_HEIGHT> =
            StackVec::new(StackVec::new(b' '));
        let mut shell = Self {
            cursor_position: 0,
            buffer,
            current_command: (0, [b' '; MAX_COMMAND_LEN]),
        };
        shell.print_prompt();
        shell
    }

    pub fn print_prompt(&mut self) {
        for c in SHELL_PROMPT.chars() {
            self.write_byte(c as u8);
        }
    }

    pub fn write_byte_to_the_command_line(&mut self, byte: u8) {
        self.write_byte(byte);
    }

    fn write_byte(&mut self, byte: u8) {
        let byte = if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
            byte
        } else {
            0xfe
        };
        match byte {
            b'\n' => self.new_line(),
            byte => {
                let cursor_position = self.cursor_position;
                if self.get_command_line_data().push_at(cursor_position, byte) {
                    self.cursor_position += 1;
                }
            }
        }
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn get_command_line_data(&mut self) -> &mut StackVec<u8, VGA_BUFFER_WIDTH> {
        self.buffer.get_mut_unsafe(COMMAND_LINE_ROW_INDEX)
    }

    fn new_line(&mut self) {
        for row in 1..VGA_BUFFER_HEIGHT {
            let (row_data, row_len) = self.buffer.get_unsafe(row).copy();
            self.buffer
                .get_mut_unsafe(row - 1)
                .copy_from(&row_data, row_len);
        }

        self.get_command_line_data().clear();

        self.cursor_position = 0;
        self.print_prompt()
    }

    fn clear_row(&mut self, row: usize) {
        self.buffer.get_mut_unsafe(row).clear();
    }

    pub fn get_data(&self) -> &StackVec<StackVec<u8, VGA_BUFFER_WIDTH>, VGA_BUFFER_HEIGHT> {
        &self.buffer
    }

    pub fn handle_backspace(
        &mut self,
    ) -> Option<(&StackVec<u8, VGA_BUFFER_WIDTH>, core::ops::Range<usize>)> {
        if self.cursor_position > SHELL_PROMPT.len() {
            self.cursor_position -= 1;
            let cursor_position = self.cursor_position;
            let line = self.get_command_line_data();
            let end_pos = line.len();

            line.pop_at(cursor_position)
                .map(|_| (&*self.get_command_line_data(), cursor_position..end_pos))
        } else {
            None
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
                if self.cursor_position < self.get_command_line_data().len() {
                    self.cursor_position += 1;
                }
            }
            _ => (),
        }
    }
}

impl Clone for Shell {
    fn clone(&self) -> Self {
        *self
    }
}

impl Write for Shell {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}
