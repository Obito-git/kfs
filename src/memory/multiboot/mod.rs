use core::mem::size_of;

//TODO: pub struct variables members should be private, and provide getters

#[repr(C)]
pub struct BootInformationHeader {
    total_size: u32,
    reserved: u32,
}

#[repr(C)]
pub struct TagHeader {
    tag_type: TagType,
    size: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum TagType {
    End = 0,                   // Marks end of tags, required as last tag
    CommandLine = 1,           // Boot command-line passed to kernel
    BootLoaderName = 2,        // Bootloader name as a string
    Modules = 3,               // List of boot modules loaded with kernel
    BasicMemInfo = 4,          // Basic memory info (in KB) - lower/upper memory
    BIOSBootDev = 5,           // BIOS boot device information
    MemoryMap = 6,             // Detailed system memory map
    VBEInfo = 7,               // VESA BIOS Extensions (VBE) info
    FrameBufferInfo = 8,       // Framebuffer info for graphics
    ELFSections = 9,           // ELF sections of kernel
    APMTable = 10,             // Advanced Power Management (APM) table
    EFI32SystemTablePtr = 11,  // 32-bit pointer to EFI system table
    EFI64SystemTablePtr = 12,  // 64-bit pointer to EFI system table
    SMBIOSTables = 13,         // System Management BIOS (SMBIOS) tables
    ACPIOldRSDP = 14,          // Old ACPI RSDP (Root System Description Pointer)
    ACPINewRSDP = 15,          // New ACPI RSDP
    NetworkingInfo = 16,       // Networking information
    EFIMemoryMap = 17,         // EFI memory map
    EFIBootServicesClear = 18, // Indicates EFI boot services not available
    EFI32ImageHandle = 19,     // 32-bit EFI image handle pointer
    EFI64ImageHandle = 20,     // 64-bit EFI image handle pointer
    LoadBaseAddr = 21,         // Image load base physical address
}

#[repr(C)]
pub struct ElfSectionsTag {
    typ: u32,
    size: u32,
    num: u32,
    entry_size: u32,
    shndx: u32,
    first_section: ElfSection,
}

#[repr(C)]
pub struct ElfSection {
    name: u32,
    typ: u32,
    pub(crate) flags: u32,        
    pub(crate) addr: u32,         
    offset: u32,       
    pub(crate) size: u32,         
    link: u32,
    info: u32,
    addralign: u32,    
    entry_size: u32,   
}

impl ElfSectionsTag {
    pub fn sections(&self) -> ElfSectionIter {
        let sections_start = (&self.first_section) as *const _;
        ElfSectionIter {
            current: sections_start,
            remaining: self.num,
            entry_size: self.entry_size,
        }
    }
}

pub struct ElfSectionIter {
    current: *const ElfSection,
    remaining: u32,
    entry_size: u32,
}

impl Iterator for ElfSectionIter {
    type Item = &'static ElfSection;

    /// Skips null sections
    fn next(&mut self) -> Option<Self::Item> {
        while self.remaining > 0 {
            let section = unsafe { &*self.current };

            self.current = unsafe {
                (self.current as *const u8)
                    .add(self.entry_size as usize) as *const ElfSection
            };
            self.remaining -= 1;

            if section.size > 0 && section.addr > 0 {
                return Some(section);
            }
        }
        None
    }
}

#[repr(C)]
pub struct MemoryMapTag {
    header: TagHeader,
    entry_size: u32,
    entry_version: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum MemoryRegionType {
    Available = 1,       // Free memory, available for OS use
    Reserved = 2,        // Reserved, don't use
    ACPIReclaimable = 3, // ACPI tables that can be reclaimed
    ACPINVS = 4,         // ACPI NVS memory, must be preserved
    BadRAM = 5,          // Memory with bad blocks, don't use
}

#[repr(C)]
pub struct MemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub region_type: MemoryRegionType,
    reserved: u32,
}

impl MemoryMapEntry {
    pub fn is_available(&self) -> bool {
        self.region_type == MemoryRegionType::Available
    }
}

#[derive(Debug)]
pub struct BootInformation {
    pub(crate) addr: *const BootInformationHeader,
    pub(crate) size: u32,
}

impl BootInformation {
    pub unsafe fn load(addr: *const BootInformationHeader) -> Option<BootInformation> {
        let header = &*addr;
        if header.reserved != 0 {
            return None;
        }
        Some(BootInformation {
            addr,
            size: header.total_size,
        })
    }

    pub fn tags(&self) -> TagIterator {
        unsafe {
            // skip first 8 bytes (BootInformationHeader)
            let first_tag = (self.addr as *const u8).add(8) as *const TagHeader;
            TagIterator {
                current: first_tag,
                end: (self.addr as *const u8).add(self.size as usize) as *const TagHeader,
            }
        }
    }

    pub fn memory_map_tag(&self) -> Option<&MemoryMapTag> {
        self.tags()
            .find(|tag| tag.tag_type == TagType::MemoryMap)
            .map(|tag| unsafe { &*(tag as *const _ as *const MemoryMapTag) })
    }

    pub fn start_address(&self) -> u32 {
        self.addr as u32
    }

    pub fn end_address(&self) -> u32 {
        (self.addr as u32) + self.size
    }

    pub fn elf_sections_tag(&self) -> Option<&ElfSectionsTag> {
        self.tags()
            .find(|tag| tag.tag_type == TagType::ELFSections)
            .map(|tag| unsafe { &*(tag as *const _ as *const ElfSectionsTag) })
    }
}

pub struct TagIterator {
    current: *const TagHeader,
    end: *const TagHeader,
}

impl Iterator for TagIterator {
    type Item = &'static TagHeader;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.current >= self.end {
                return None;
            }

            let tag = &*self.current;
            if tag.tag_type == TagType::End {
                return None;
            }

            // Calculate next tag address (8-byte aligned)
            let next = ((self.current as usize + tag.size as usize + 7) & !7) as *const TagHeader;
            self.current = next;

            Some(tag)
        }
    }
}

impl MemoryMapTag {
    pub fn memory_areas(&self) -> MemoryAreaIter {
        let entries_addr = (self as *const _) as usize + size_of::<MemoryMapTag>();
        let entries_count = (self.header.size as usize - size_of::<MemoryMapTag>()) / size_of::<MemoryMapEntry>();
        let areas = unsafe {
            core::slice::from_raw_parts(entries_addr as *const MemoryMapEntry, entries_count)
        };
        MemoryAreaIter { areas, index: 0 }
    }
}


#[derive(Clone)]
pub struct MemoryAreaIter<'a> {
    areas: &'a [MemoryMapEntry],
    index: usize,
}

impl<'a> Iterator for MemoryAreaIter<'a> {
    type Item = &'a MemoryMapEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.areas.len() {
            let entry = &self.areas[self.index];
            self.index += 1;
            Some(entry)
        } else {
            None
        }
    }
}
