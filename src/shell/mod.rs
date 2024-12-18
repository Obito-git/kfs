mod command;

use crate::data_structure::StackVec;
use crate::io::vga::{SHELL_BUFFER_HEIGHT, SHELL_COMMAND_LINE_ROW_INDEX, VGA_BUFFER_WIDTH};
use core::fmt;
use core::fmt::Write;
use crate::io::keyboard::NavigationKey;
use crate::shell::command::Command;

pub const SHELL_PROMPT: &str = "$> ";
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

#[derive(Debug, Copy)]
pub struct Shell {
    cursor_position: usize,
    buffer: StackVec<StackVec<u8, VGA_BUFFER_WIDTH>, SHELL_BUFFER_HEIGHT>,
}

impl Shell {
    pub fn new() -> Self {
        let buffer: StackVec<StackVec<u8, VGA_BUFFER_WIDTH>, SHELL_BUFFER_HEIGHT> =
            StackVec::new(StackVec::new(b' '));
        let mut shell = Self {
            cursor_position: 0,
            buffer,
        };
        shell.print_header();
        shell.print_prompt();
        shell
    }

    pub fn print_prompt(&mut self) {
        for c in SHELL_PROMPT.chars() {
            self.write_byte(c as u8);
        }
    }

    pub fn print_header(&mut self) {
        self.write_str(HEADER).unwrap();
    }

    pub fn write_byte_to_the_command_line(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                let command =
                    Command::try_from(self.buffer.get_unsafe(SHELL_COMMAND_LINE_ROW_INDEX));
                self.new_line();
                match command {
                    Ok(command) => {
                        let handler = command.get_handler();
                        handler(self);
                    }
                    Err(msg) => self.write_str(msg).unwrap(),
                }
                self.new_line();
                self.print_prompt();
            }
            byte => self.write_byte(byte),
        }
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
                let cmd_line = self.buffer.get_mut_unsafe(SHELL_COMMAND_LINE_ROW_INDEX);
                if cmd_line.push_at(cursor_position, byte) {
                    self.cursor_position += 1;
                }
            }
        }
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    fn new_line(&mut self) {
        for row in 1..SHELL_BUFFER_HEIGHT {
            let (row_data, row_len) = self.buffer.get_unsafe(row).copy();
            self.buffer
                .get_mut_unsafe(row - 1)
                .copy_from(&row_data, row_len);
        }

        let cmd_line = self.buffer.get_mut_unsafe(SHELL_COMMAND_LINE_ROW_INDEX);
        cmd_line.clear();

        self.cursor_position = 0;
    }

    fn clear_buffer(&mut self) {
        self.buffer = StackVec::new(StackVec::new(b' '));
        self.cursor_position = 0;
    }

    pub fn get_data(&self) -> &StackVec<StackVec<u8, VGA_BUFFER_WIDTH>, SHELL_BUFFER_HEIGHT> {
        &self.buffer
    }

    pub fn handle_backspace(
        &mut self,
    ) -> Option<(&StackVec<u8, VGA_BUFFER_WIDTH>, core::ops::Range<usize>)> {
        if self.cursor_position > SHELL_PROMPT.len() {
            self.cursor_position -= 1;
            let cursor_position = self.cursor_position;
            let cmd_line = self.buffer.get_mut_unsafe(SHELL_COMMAND_LINE_ROW_INDEX);
            let end_pos = cmd_line.len();

            cmd_line.pop_at(cursor_position).map(|_| {
                (
                    &*self.buffer.get_mut_unsafe(SHELL_COMMAND_LINE_ROW_INDEX),
                    cursor_position..end_pos,
                )
            })
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
                let cmd_line = self.buffer.get_unsafe(SHELL_COMMAND_LINE_ROW_INDEX);
                if self.cursor_position < cmd_line.len() {
                    self.cursor_position += 1;
                }
            }
            NavigationKey::Home => self.cursor_position = SHELL_PROMPT.len(),
            NavigationKey::End => {
                self.cursor_position = self.buffer.get_unsafe(SHELL_COMMAND_LINE_ROW_INDEX).len();
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
