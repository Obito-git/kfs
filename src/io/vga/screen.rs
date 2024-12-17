use crate::data_structure::StackVec;
use crate::io::vga::color::ColorCode;
use crate::io::{hide_cursor, set_cursor_position, show_cursor};
use crate::io::vga::{VGA_BUFFER_HEIGHT, VGA_COMMAND_LINE_ROW_INDEX, VGA_MIN_FIRST_LINE};
use crate::shell::Shell;

pub(crate) struct VgaScreen {
    pub(crate) current_buffer_first_line: usize,
    pub(crate) is_scrolling_mode_enabled: bool,
    pub(crate) shell: Shell,
    pub(crate) color: ColorCode,
}

impl VgaScreen {
    pub fn new(color: ColorCode) -> Self {
        Self {
            shell: Shell::new(),
            current_buffer_first_line: VGA_MIN_FIRST_LINE,
            is_scrolling_mode_enabled: false,
            color,
        }
    }

    pub fn get_data(&self) -> &[StackVec<u8, 80>] {
        self.shell.get_data().slice(
            self.current_buffer_first_line..self.current_buffer_first_line + VGA_BUFFER_HEIGHT,
        )
    }

    pub fn render_cursor(&mut self) {
        if !self.is_scrolling_mode_enabled {
            set_cursor_position(
                self.shell.cursor_position() as u8,
                VGA_COMMAND_LINE_ROW_INDEX as u8,
            );
        }
    }

    pub fn enable_scrolling(&mut self) {
        if !self.is_scrolling_mode_enabled {
            self.is_scrolling_mode_enabled = true;
            hide_cursor()
        }
    }

    pub fn disable_scrolling(&mut self) {
        if self.is_scrolling_mode_enabled {
            self.is_scrolling_mode_enabled = false;
            show_cursor()
        }
    }
}