use crate::memory::multiboot::{BootInformation, MemoryAreaIter, MemoryMapEntry, MemoryRegionType};
use crate::memory::{Frame, PAGE_SIZE};

pub struct BumpFrameAllocator<'a> {
    next_free_frame_number: usize,
    current_area: Option<&'a MemoryMapEntry>,
    areas: MemoryAreaIter<'a>,
    kernel_start_frame: usize,
    kernel_end_frame: usize,
    multiboot_start_frame: usize,
    multiboot_end_frame: usize,
}

impl<'a> BumpFrameAllocator<'a> {
    pub fn new(
        kernel_start: usize,
        kernel_end: usize,
        multiboot_start: usize,
        multiboot_end: usize,
        memory_areas: MemoryAreaIter<'a>,
    ) -> BumpFrameAllocator<'a> {
        let mut allocator = BumpFrameAllocator {
            next_free_frame_number: 0,
            current_area: None,
            areas: memory_areas,
            kernel_start_frame: kernel_start / PAGE_SIZE,
            kernel_end_frame: kernel_end / PAGE_SIZE,
            multiboot_start_frame: multiboot_start / PAGE_SIZE,
            multiboot_end_frame: multiboot_end / PAGE_SIZE,
        };
        allocator.choose_next_area();
        allocator
    }

    fn choose_next_area(&mut self) {
        self.current_area = self.areas.find(|area| {
            let area_end_frame = (area.base_addr as usize + area.length as usize) / PAGE_SIZE;
            area_end_frame > self.next_free_frame_number
                && area.region_type == MemoryRegionType::Available
        });

        if let Some(area) = self.current_area {
            let area_start_frame = area.base_addr as usize / PAGE_SIZE;
            if self.next_free_frame_number < area_start_frame {
                self.next_free_frame_number = area_start_frame;
            }
        }
    }

    pub(crate) fn allocate_frame(&mut self) -> Option<Frame> {
        if let Some(area) = self.current_area {
            let frame_number = self.next_free_frame_number;

            let area_end_frame = (area.base_addr as usize + area.length as usize) / PAGE_SIZE;

            if frame_number >= area_end_frame {
                self.choose_next_area();
                return self.allocate_frame();
            }

            if frame_number >= self.kernel_start_frame && frame_number <= self.kernel_end_frame {
                self.next_free_frame_number = self.kernel_end_frame + 1;
                return self.allocate_frame();
            }

            if frame_number >= self.multiboot_start_frame
                && frame_number <= self.multiboot_end_frame
            {
                self.next_free_frame_number = self.multiboot_end_frame + 1;
                return self.allocate_frame();
            }

            self.next_free_frame_number += 1;
            Some(Frame {
                number: frame_number,
            })
        } else {
            None
        }
    }

    fn deallocate_frame(&mut self, _frame: Frame) {
        unimplemented!();
    }
}

impl<'a> From<&'a BootInformation> for BumpFrameAllocator<'a> {
    fn from(boot_info: &'a BootInformation) -> Self {
        let kernel_start = boot_info
            .elf_sections_tag()
            .expect("ELF sections tag required")
            .sections()
            .map(|s| s.addr)
            .min()
            .unwrap() as usize;

        let kernel_end = boot_info
            .elf_sections_tag()
            .expect("ELF sections tag required")
            .sections()
            .map(|s| s.addr + s.size)
            .max()
            .unwrap() as usize;

        let multiboot_start = boot_info.addr as usize;
        let multiboot_end = multiboot_start + boot_info.size as usize;

        let memory_areas = boot_info
            .memory_map_tag()
            .expect("Memory map tag required")
            .memory_areas();

        BumpFrameAllocator::new(
            kernel_start,
            kernel_end,
            multiboot_start,
            multiboot_end,
            memory_areas,
        )
    }
}
