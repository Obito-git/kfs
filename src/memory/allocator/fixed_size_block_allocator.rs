use crate::memory::paging::{map_page, EntryFlags, VirtualAddress};
use crate::memory::FRAME_ALLOCATOR;
use alloc::alloc::{GlobalAlloc, Layout};
use crate::println;

#[global_allocator]
static ALLOCATOR: FixedSizeBlockAllocator = FixedSizeBlockAllocator;

pub const KERNEL_HEAP_START: u32 = 0xC010_0000; // Start of the heap
pub const KERNEL_HEAP_SIZE: u32 = 0x0010_0000; // 1MB heap size

pub struct FixedSizeBlockAllocator;

unsafe impl GlobalAlloc for FixedSizeBlockAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!("Entered alloc");
        // Get the size and align it to the page size
        let size = layout.size().max(crate::memory::PAGE_SIZE);
        let align = layout.align().max(crate::memory::PAGE_SIZE);

        // Get a virtual address range for the allocation
        static mut NEXT_VIRTUAL_ADDRESS: usize = 0xC000_0000; // Start of heap
        let virtual_address = NEXT_VIRTUAL_ADDRESS;
        NEXT_VIRTUAL_ADDRESS += size;

        // Allocate and map physical frames
        for offset in (0..size).step_by(crate::memory::PAGE_SIZE) {
            println!("Entered loop");
            let frame = FRAME_ALLOCATOR
                .lock()
                .allocate_frame()
                .expect("Out of physical memory");
            println!("Passed lock");
            let physical_address = frame.start_address() as u32;
            let virtual_page = VirtualAddress::new((virtual_address + offset) as u32);

            map_page(
                virtual_page,
                physical_address,
                EntryFlags::WRITABLE | EntryFlags::PRESENT,
            );
        }

        virtual_address as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        //panic!("dealloc should be never called")
        println!("Dealloc");
    }
}
