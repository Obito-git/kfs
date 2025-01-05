use crate::memory::allocator::bump_frame_allocator::BumpFrameAllocator;
use crate::memory::FRAME_ALLOCATOR;
use bitflags::bitflags;
use core::arch::asm;
use core::ptr::write_volatile;
use lazy_static::lazy_static;
use spin::mutex::Mutex;

pub const PAGE_SIZE: usize = 4096;

lazy_static! {
    static ref KERNEL_PAGE_DIRECTORY: Mutex<PageDirectory> = Mutex::new(PageDirectory {
        entries: [PageTableEntry::zero(); 1024],
    });
}

bitflags! {
    pub struct EntryFlags: u32 {
        const PRESENT =         1 << 0;
        const WRITABLE =        1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH =   1 << 3;
        const NO_CACHE =        1 << 4;
        const ACCESSED =        1 << 5;
        const DIRTY =           1 << 6;
        const HUGE_PAGE =       1 << 7;
        const GLOBAL =          1 << 8;
        const AVL_1 =           1 << 9;
        const AVL_2 =           1 << 10;
        const AVL_3 =           1 << 11;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VirtualAddress(u32);

impl VirtualAddress {
    pub fn new(address: u32) -> Self {
        VirtualAddress(address)
    }
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    // bits 31-22
    pub fn directory_index(&self) -> usize {
        ((self.0 >> 22) & 0x3FF) as usize
    }

    // bits 21-12
    pub fn table_index(&self) -> usize {
        ((self.0 >> 12) & 0x3FF) as usize
    }

    // bits 11-0
    pub fn offset(&self) -> usize {
        (self.0 & 0xFFF) as usize
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageTableEntry(pub(crate) u32);

impl PageTableEntry {
    pub fn new(addr: u32, flags: EntryFlags) -> Self {
        PageTableEntry((addr & 0xFFFFF000) | flags.bits())
    }

    pub fn zero() -> Self {
        PageTableEntry(0)
    }

    pub fn addr(&self) -> u32 {
        self.0 & 0xFFFFF000
    }

    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0 & 0xFFF)
    }

    pub fn set_addr(&mut self, addr: u32) {
        let flags = self.0 & 0xFFF;
        self.0 = (addr & 0xFFFFF000) | flags;
    }

    pub fn set_flags(&mut self, flags: EntryFlags) {
        let addr = self.0 & 0xFFFFF000;
        self.0 = addr | flags.bits();
    }

    pub fn is_present(&self) -> bool {
        self.flags().contains(EntryFlags::PRESENT)
    }
}

#[repr(align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 1024],
}

#[repr(align(4096))]
pub struct PageDirectory {
    pub(crate) entries: [PageTableEntry; 1024],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page {
    number: u32,
}

impl Page {
    pub fn containing_address(address: u32) -> Page {
        Page {
            number: address / PAGE_SIZE as u32,
        }
    }

    pub fn start_address(&self) -> u32 {
        self.number * PAGE_SIZE as u32
    }

    // For 32-bit paging:
    pub fn directory_index(&self) -> usize {
        ((self.number >> 10) & 0x3FF) as usize // bits 31-22
    }

    pub fn table_index(&self) -> usize {
        (self.number & 0x3FF) as usize // bits 21-12
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalAddress(u32);

impl PhysicalAddress {
    pub fn new(address: u32) -> Self {
        PhysicalAddress(address)
    }
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

/// Creates an identity-mapped paging structure.
/// Maps physical addresses 0x00000000 to 0x003FFFFF (first 4 MB).
pub fn setup_identity_mapping() {
    unsafe {
        let pt_frame = FRAME_ALLOCATOR
            .lock()
            .allocate_frame()
            .expect("No frame available for Page Table");
        let pt_address = pt_frame.start_address() as *mut PageTable;

        write_volatile(
            pt_address,
            PageTable {
                entries: [PageTableEntry::zero(); 1024],
            },
        );

        let page_table = &mut *pt_address;

        for i in 0..1024 {
            let frame_address = (i * crate::memory::PAGE_SIZE) as u32;
            let entry =
                PageTableEntry::new(frame_address, EntryFlags::PRESENT | EntryFlags::WRITABLE);
            page_table.entries[i] = entry;
        }

        let pt_physical_address = pt_frame.start_address() as u32;
        KERNEL_PAGE_DIRECTORY.lock().entries[0] = PageTableEntry::new(
            pt_physical_address,
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );
    }
}

pub fn map_page(
    virtual_address: VirtualAddress,
    physical_address: u32,
    flags: EntryFlags,
) {
    // Get the directory and table indices
    let directory_index = virtual_address.directory_index();
    let table_index = virtual_address.table_index();

    // Check if the page table is already present
    let mut page_directory = KERNEL_PAGE_DIRECTORY.lock();
    if page_directory.entries[directory_index].is_present() {
        // Get the physical address of the page table
        let table_physical_address = page_directory.entries[directory_index].addr();
        let table = unsafe { &mut *(table_physical_address as *mut PageTable) };

        // Map the page in the existing table
        table.entries[table_index] =
            PageTableEntry::new(physical_address, flags | EntryFlags::PRESENT);
    } else {
        // Allocate a new frame for the page table
        let frame = FRAME_ALLOCATOR.lock()
            .allocate_frame()
            .expect("Out of memory for page table!");
        let new_table_address = frame.start_address() as *mut PageTable;

        // Initialize the new page table
        unsafe {
            new_table_address.write(PageTable {
                entries: [PageTableEntry::zero(); 1024],
            });
        }
        let new_table = unsafe { &mut *new_table_address };

        // Map the new table into the page directory
        page_directory.entries[directory_index] = PageTableEntry::new(
            frame.start_address() as u32,
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        );

        // Map the page in the new table
        new_table.entries[table_index] =
            PageTableEntry::new(physical_address, flags | EntryFlags::PRESENT);
    }
}

pub unsafe fn enable_paging() {
    setup_identity_mapping();
    let pd_physical_address = KERNEL_PAGE_DIRECTORY.lock().entries.as_ptr() as u32;

    asm!(
    "mov cr3, {0}",         // Load page directory base address into CR3
    "mov eax, cr0",         // Load current CR0
    "or eax, 0x80000000",   // Set the PG (Paging Enable) bit
    "mov cr0, eax",         // Write updated value back to CR0
    in(reg) pd_physical_address,
    options(nostack, preserves_flags)
    );
}
