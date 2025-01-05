use crate::memory::multiboot::{BootInformation, MemoryMapEntry, MemoryRegionType};
use crate::memory::{Frame, PAGE_SIZE};

const MAX_MEMORY_AREAS: usize = 32;
const MAX_FREED_FRAMES: usize = 1024;

/// Very naive implementation of Physical memory allocator with huge limitations
/// but it is more than enough for this KFS stage
pub struct BumpFrameAllocator {
    next_free_frame_number: usize,
    current_area_index: usize,
    areas: [Option<MemoryMapEntry>; MAX_MEMORY_AREAS],
    num_areas: usize,
    kernel_start_frame: usize,
    kernel_end_frame: usize,
    multiboot_start_frame: usize,
    multiboot_end_frame: usize,
    freed_frames: [Option<Frame>; MAX_FREED_FRAMES],
    num_freed_frames: usize,
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

        let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
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
            freed_frames: [None; MAX_FREED_FRAMES],
            num_freed_frames: 0,
        };

        allocator.choose_next_area();
        allocator
    }

    fn choose_next_area(&mut self) {
        while self.current_area_index < self.num_areas {
            let area = self.areas[self.current_area_index].expect("Memory area must be present");
            let area_start_frame = area.base_addr as usize / PAGE_SIZE;
            let area_end_frame = area_start_frame + (area.length as usize / PAGE_SIZE);

            if area.region_type == MemoryRegionType::Available
                && area_end_frame > self.next_free_frame_number
            {
                if self.next_free_frame_number < area_start_frame {
                    self.next_free_frame_number = area_start_frame;
                }
                return;
            }
            self.current_area_index += 1;
        }
    }

    pub fn allocate_frame(&mut self) -> Option<Frame> {
        if self.num_freed_frames > 0 {
            self.num_freed_frames -= 1;
            return self.freed_frames[self.num_freed_frames].take();
        }

        // If we’re past the last area, no memory left
        if self.current_area_index >= self.num_areas {
            return None;
        }

        let area = self.areas[self.current_area_index].expect("Memory area unexpectedly None?");
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
        Some(Frame {
            number: frame_number,
        })
    }

    pub fn deallocate_frame(&mut self, frame: Frame) {
        if self.num_freed_frames < MAX_FREED_FRAMES {
            self.freed_frames[self.num_freed_frames] = Some(frame);
            self.num_freed_frames += 1;
        } else {
            panic!("Frame allocator freed_frames overflow!");
        }
    }
}
