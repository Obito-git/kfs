pub mod multiboot;
pub mod bump_frame_allocator;
pub mod paging;
mod heap;

pub const PAGE_SIZE: usize = 4096;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    pub(crate) fn containing_address(address: usize) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }

    pub(crate) fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }
}

