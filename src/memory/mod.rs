use crate::memory::bump_frame_allocator::BumpFrameAllocator;
use crate::memory::multiboot::{BootInformation, BootInformationHeader};
use lazy_static::lazy_static;
use spin::lock_api::Mutex;

pub mod bump_frame_allocator;
mod heap;
pub mod multiboot;
pub mod paging;

extern "C" {
    static multiboot_info_addr: u32;
}

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<BumpFrameAllocator> = {
        unsafe {
            let boot_info =
                BootInformation::load(multiboot_info_addr as *const BootInformationHeader)
                    .expect("Invalid boot information");
            Mutex::new(BumpFrameAllocator::new(boot_info))
        }
    };
}

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    pub(crate) fn containing_address(address: usize) -> Frame {
        Frame {
            number: address / PAGE_SIZE,
        }
    }

    pub(crate) fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn clone(&self) -> Frame {
        Frame {
            number: self.number,
        }
    }
}
