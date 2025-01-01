use crate::{kernel_panic, println};
use core::arch::asm;

const PAGE_FAULT_VECTOR_VALUE: u8 = 14;

/// The stack frame pushed by the CPU during an interrupt.
#[repr(C)]
pub struct InterruptStackFrame {
    pub eip: u32,    // Instruction Pointer
    pub cs: u32,     // Code Segment
    pub eflags: u32, // Flags Register
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IdtEntry {
    offset_low: u16,  // Lower 16 bits of the handler's address
    selector: u16,    // Segment selector
    zero: u8,         // Reserved (must be zero)
    attributes: u8,   // Type and attributes
    offset_high: u16, // Higher 16 bits of the handler's address
}

impl IdtEntry {
    pub fn new(handler: u32, selector: u16, attributes: u8) -> Self {
        IdtEntry {
            offset_low: (handler & 0xFFFF) as u16,
            selector,
            zero: 0,
            attributes,
            offset_high: (handler >> 16) as u16,
        }
    }
}

#[repr(C, align(8))]
pub struct Idt {
    entries: [IdtEntry; 256],
}

impl Idt {
    pub fn new() -> Self {
        Idt {
            entries: [IdtEntry {
                offset_low: 0,
                selector: 0,
                zero: 0,
                attributes: 0,
                offset_high: 0,
            }; 256],
        }
    }

    pub fn set_handler(&mut self, index: u8, handler: u32, selector: u16, attributes: u8) {
        self.entries[index as usize] = IdtEntry::new(handler, selector, attributes);
    }

    pub fn load(&self) {
        let idt_pointer = IdtPointer {
            limit: (size_of::<Idt>() - 1) as u16,
            base: self as *const _ as u32,
        };

        unsafe {
            asm!(
            "lidt [{}]",
            in(reg) &idt_pointer,
            options(readonly, nostack, preserves_flags)
            );
            println!("IDT loaded!");
        }
    }
}

#[repr(C, packed)]
struct IdtPointer {
    limit: u16,
    base: u32,
}

#[no_mangle]
extern "C" fn page_fault_handler(_stack_frame: &InterruptStackFrame, error_code: u32) {
    let faulting_address: u32;
    unsafe {
        asm!("mov {}, cr2", out(reg) faulting_address);
    }

    if error_code & 0x1 == 0 {
        kernel_panic!(
            fatal,
            "Page not present at address 0x{:x}, error code: 0x{:x}",
            faulting_address,
            error_code
        );
    } else {
        kernel_panic!(recoverable, "Page fault in user mode.");
    }
}

pub unsafe fn enable_interrupts() {
    initialize_idt();
    asm!("sti", options(nostack, preserves_flags));
}

fn mask_pic_interrupts() {
    let mask: u8 = 0xFF;
    unsafe {
        asm!("out 0x21, al", in("al") mask);
        asm!("out 0xA1, al", in("al") mask);
    }
}

fn initialize_idt() {
    let mut idt = Idt::new();

    idt.set_handler(
        PAGE_FAULT_VECTOR_VALUE,
        page_fault_handler as u32,
        0x08,
        0x8E,
    );

    // Mask all other interrupts
    mask_pic_interrupts();

    // Load the IDT
    idt.load();
}
