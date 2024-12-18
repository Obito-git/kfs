mod print_stack;

use crate::data_structure::StackVec;
use crate::io::vga::VGA_BUFFER_WIDTH;
use crate::io::{exit_qemu, reboot};
use crate::shell::command::print_stack::{hexdump_stack, test_stack};
use crate::shell::{Shell, SHELL_PROMPT};

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Reboot,
    Clear,
    PowerOff,
    PrintKernelStack,
    PrintTestStack,
}

impl Command {
    const COMMANDS: &'static [(&'static str, Command)] = &[
        ("reboot", Command::Reboot),
        ("clear", Command::Clear),
        ("poweroff", Command::PowerOff),
        ("pks", Command::PrintKernelStack),
        ("pts", Command::PrintTestStack),
    ];

    pub fn get_handler(&self) -> fn(&mut Shell) {
        match self {
            Command::Reboot => |_shell| reboot(),
            Command::Clear => Shell::clear_buffer,
            Command::PowerOff => |_shell| exit_qemu(),
            Command::PrintKernelStack => |shell| hexdump_stack(shell),
            Command::PrintTestStack => |shell| test_stack(shell),
        }
    }
}

impl TryFrom<&StackVec<u8, VGA_BUFFER_WIDTH>> for Command {
    type Error = &'static str;
    fn try_from(value: &StackVec<u8, VGA_BUFFER_WIDTH>) -> Result<Self, Self::Error> {
        for (command_name, cmd) in Self::COMMANDS {
            if command_name.as_bytes() == value.slice(SHELL_PROMPT.len()..value.len()) {
                return Ok(*cmd);
            }
        }
        Err("command not found")
    }
}
