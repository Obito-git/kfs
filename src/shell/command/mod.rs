mod print_stack;

use core::fmt::Write;

use crate::data_structure::StackVec;
use crate::io::vga::VGA_BUFFER_WIDTH;
use crate::io::{exit_qemu, reboot};
use crate::shell::command::print_stack::{hexdump_stack, test_stack};
use crate::shell::{Shell, SHELL_PROMPT};
use lazy_static::lazy_static;

extern "C" {
    static gdt_start: usize;
    static gdt_ptr: usize;
    static stack_start: usize; // top of the stack (highest address)
    static stack_end: usize;
}

lazy_static! {
    static ref GDT_START: usize = unsafe { &gdt_start as *const usize as usize };
    static ref GDT_POINTER: usize = unsafe { &gdt_ptr as *const usize as usize };

}

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Reboot,
    Clear,
    PowerOff,
    PrintKernelStack,
    PrintTestStack,
    PrintGdt,
}

impl Command {
    const COMMANDS: &'static [(&'static str, Command)] = &[
        ("reboot", Command::Reboot),
        ("clear", Command::Clear),
        ("poweroff", Command::PowerOff),
        ("pks", Command::PrintKernelStack),
        ("pts", Command::PrintTestStack),
        ("pgdt", Command::PrintGdt)
    ];

    pub fn get_handler(&self) -> fn(&mut Shell) {
        match self {
            Command::Reboot => |_shell| reboot(),
            Command::Clear => Shell::clear_buffer,
            Command::PowerOff => |_shell| exit_qemu(),
            Command::PrintKernelStack => |shell| hexdump_stack(shell),
            Command::PrintTestStack => |shell| test_stack(shell),
            Command::PrintGdt => |shell| {
                for address in (*GDT_START..*GDT_POINTER).step_by(8) {
                    shell.write_fmt(format_args!("{:#07x}:", address)).unwrap();
                    for i in 0..8 {
                        let value = unsafe { *((address + i) as *const u8) };
                        shell.write_fmt(format_args!(" {value:08b}")).unwrap();
                    }
                    shell.new_line();
                }
            }
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
