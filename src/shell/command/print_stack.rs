use crate::io::get_esp;
use crate::shell::Shell;
use core::fmt::Write;

const BYTES_PER_LINE: usize = 16;

extern "C" {
    static stack_start: usize; // top of the stack (highest address)
    static stack_end: usize; // bottom of the stack (lowest address)
}

#[no_mangle]
pub fn test_stack(shell: &mut Shell) {
    let string = b"HELLO FROM STACK!";
    let mut stack_str = [0u8; 32];
    stack_str[..string.len()].copy_from_slice(string);

    let dead_beef1: u32 = 0xDEADBEEF;
    let dead_beef2 = [0xDE_u8, 0xAD_u8, 0xBE_u8, 0xEF_u8];

    // force compiler to keep these variables
    core::hint::black_box(&stack_str);
    core::hint::black_box(&dead_beef1);
    core::hint::black_box(&dead_beef2);

    hexdump_stack(shell);
}

pub fn hexdump_stack(shell: &mut Shell) {
    let stack_top_addr = unsafe { &stack_start as *const usize as usize };
    let stack_bottom_addr = unsafe { &stack_end as *const usize as usize };
    let esp = get_esp();

    let mut current = stack_bottom_addr;
    let mut last_line = [0u8; BYTES_PER_LINE];
    let mut repeated = false;

    while current < stack_top_addr {
        let mut this_line = [0u8; BYTES_PER_LINE];
        let bytes_remaining = stack_top_addr - current;
        let bytes_to_read = core::cmp::min(BYTES_PER_LINE, bytes_remaining);

        for i in 0..bytes_to_read {
            this_line[i] = unsafe { *((current + i) as *const u8) };
        }

        if this_line == last_line && current != stack_bottom_addr {
            if !repeated {
                shell.write_str("*\n").unwrap();
                repeated = true;
            }
        } else {
            repeated = false;
            shell
                .write_fmt(format_args!("0x{:08x}:  ", current))
                .unwrap();

            for i in 0..BYTES_PER_LINE {
                if i < bytes_to_read {
                    shell
                        .write_fmt(format_args!("{:02x} ", this_line[i]))
                        .unwrap();
                } else {
                    shell.write_str("   ").unwrap();
                }

                if i == 7 {
                    shell.write_str(" ").unwrap();
                }
            }

            shell.write_str(" |").unwrap();
            for i in 0..bytes_to_read {
                let c = this_line[i];
                let printable = if c.is_ascii_graphic() || c == b' ' {
                    c
                } else {
                    b'.'
                };
                shell.write_byte(printable);
            }
            shell.write_str("|\n").unwrap();

            last_line = this_line;
        }

        current += BYTES_PER_LINE;
    }

    shell.write_fmt(format_args!("\nStack dump:\n")).unwrap();
    shell
        .write_fmt(format_args!("ESP: 0x{:08x}\n", esp))
        .unwrap();
    shell
        .write_fmt(format_args!("Stack bottom: 0x{:08x}\n", stack_bottom_addr))
        .unwrap();
    shell
        .write_fmt(format_args!("Stack top: 0x{:08x}\n", stack_top_addr))
        .unwrap();
}
