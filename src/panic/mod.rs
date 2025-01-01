use crate::println;
use core::arch::asm;
use core::fmt;
use core::panic::PanicInfo;

pub enum PanicSeverity {
    Fatal,
    Recoverable,
}

pub fn kernel_panic(args: fmt::Arguments, severity: PanicSeverity) {
    match severity {
        PanicSeverity::Fatal => panic!("{}", args),
        PanicSeverity::Recoverable => {
            println!("[KERNEL PANIC] Recoverable: {}", args);
            // TODO
        }
    }
}

#[macro_export]
macro_rules! kernel_panic {
    (fatal, $($arg:tt)*) => {
        $crate::panic::kernel_panic(
            core::format_args!($($arg)*),
            $crate::panic::PanicSeverity::Fatal
        );
    };
    (recoverable, $($arg:tt)*) => {
        $crate::panic::kernel_panic(
            core::format_args!($($arg)*),
            $crate::panic::PanicSeverity::Recoverable
        );
    };
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Custom panic handler: {}", info);

    loop {
        unsafe { asm!("hlt", options(nostack, preserves_flags)) }
    }
}
