use crate::memory::multiboot::{
    BootInformation,
    MemoryMapEntry,
    MemoryRegionType,
};
use crate::memory::{Frame, PAGE_SIZE};

const MAX_MEMORY_AREAS: usize = 32;

pub struct BumpFrameAllocator {
    next_free_frame_number: usize,
    current_area_index: usize,
    areas: [Option<MemoryMapEntry>; MAX_MEMORY_AREAS],
    num_areas: usize,
    kernel_start_frame: usize,
    kernel_end_frame: usize,
    multiboot_start_frame: usize,
    multiboot_end_frame: usize,
}

impl BumpFrameAllocator {
    pub fn new(boot_info: BootInformation) -> Self {
        let elf_tag = boot_info
            .elf_sections_tag()
            .expect("ELF sections tag required");

        let kernel_start = elf_tag
            .sections()
            .map(|s| s.addr as usize)
            .min()
            .expect("Couldn’t find kernel start");
        let kernel_end = elf_tag
            .sections()
            .map(|s| (s.addr + s.size) as usize)
            .max()
            .expect("Couldn’t find kernel end");

        let multiboot_start = boot_info.addr as usize;
        let multiboot_end = multiboot_start + boot_info.size as usize;

        let memory_map_tag = boot_info
            .memory_map_tag()
            .expect("Memory map tag required");
        let mut memory_areas_iter = memory_map_tag.memory_areas();

        let mut areas = [None; MAX_MEMORY_AREAS];
        let mut count = 0;
        for entry in &mut memory_areas_iter {
            if count >= MAX_MEMORY_AREAS {
                panic!("Too many memory areas (increase MAX_MEMORY_AREAS).");
            }
            areas[count] = Some(*entry);
            count += 1;
        }

        let mut allocator = BumpFrameAllocator {
            next_free_frame_number: 0,
            current_area_index: 0,
            areas,
            num_areas: count,
            kernel_start_frame: kernel_start / PAGE_SIZE,
            kernel_end_frame: kernel_end / PAGE_SIZE,
            multiboot_start_frame: multiboot_start / PAGE_SIZE,
            multiboot_end_frame: multiboot_end / PAGE_SIZE,
        };

        allocator.choose_next_area();
        allocator
    }

    /// Move `current_area_index` to the next memory area that can still serve `next_free_frame_number`.
    fn choose_next_area(&mut self) {
        while self.current_area_index < self.num_areas {
            let area = self.areas[self.current_area_index]
                .expect("Memory area must be present");
            let area_start_frame = area.base_addr as usize / PAGE_SIZE;
            let area_end_frame = area_start_frame + (area.length as usize / PAGE_SIZE);

            // Must also check that the area is "Available" (if that’s how your BIOS marks free memory)
            if area.region_type == MemoryRegionType::Available
                && area_end_frame > self.next_free_frame_number
            {
                // If our next_free_frame_number is below this area’s start, jump forward
                if self.next_free_frame_number < area_start_frame {
                    self.next_free_frame_number = area_start_frame;
                }
                return;
            }
            self.current_area_index += 1;
        }
    }

    /// Allocate a frame, if possible, returning `Some(Frame)` or `None`.
    pub fn allocate_frame(&mut self) -> Option<Frame> {
        // If we’re past the last area, no memory left
        if self.current_area_index >= self.num_areas {
            return None;
        }

        let area = self.areas[self.current_area_index]
            .expect("Memory area unexpectedly None?");
        let area_start_frame = area.base_addr as usize / PAGE_SIZE;
        let area_end_frame = area_start_frame + (area.length as usize / PAGE_SIZE);

        // If next_free_frame_number is already past the end of this area, move on
        if self.next_free_frame_number >= area_end_frame {
            self.current_area_index += 1;
            self.choose_next_area();
            return self.allocate_frame();
        }

        // Skip kernel region
        if self.next_free_frame_number >= self.kernel_start_frame
            && self.next_free_frame_number <= self.kernel_end_frame
        {
            self.next_free_frame_number = self.kernel_end_frame + 1;
            return self.allocate_frame();
        }

        // Skip multiboot region
        if self.next_free_frame_number >= self.multiboot_start_frame
            && self.next_free_frame_number <= self.multiboot_end_frame
        {
            self.next_free_frame_number = self.multiboot_end_frame + 1;
            return self.allocate_frame();
        }

        // Otherwise, we can allocate the next_free_frame_number
        let frame_number = self.next_free_frame_number;
        self.next_free_frame_number += 1;
        Some(Frame { number: frame_number })
    }

    /// For completeness: Deallocate is not implemented here.
    pub fn deallocate_frame(&mut self, _frame: Frame) {
        unimplemented!();
    }
}

// Optionally, implement the From trait so you can do `BumpFrameAllocator::from(boot_info)`.
impl From<BootInformation> for BumpFrameAllocator {
    fn from(boot_info: BootInformation) -> Self {
        BumpFrameAllocator::new(boot_info)
    }
}
