use crate::io::get_esp;
use crate::shell::Shell;
use core::fmt::Write;

extern "C" {
    static stack_top: u32;
}

pub fn print_stack(shell: &mut Shell, num_entries: usize) {
    unsafe {
        let esp = get_esp();
        let top = &stack_top as *const u32 as u32;
        shell.write_fmt(format_args!("Stack size: {} bytes\n", top - esp));

        shell.write_str("Stack Trace:\n");
        shell.write_str("Address     | Value\n");
        shell.write_str("-----------+-----------\n");


        for i in 0..num_entries {
            let addr = esp + (i * 4) as u32; // Each entry is 4 bytes
            let value = *(addr as *const u32);
            shell.write_fmt(format_args!("0x{:08x} | 0x{:08x}\n", addr, value));
        }
    }
}
