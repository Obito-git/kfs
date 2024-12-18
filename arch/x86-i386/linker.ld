ENTRY(start)

SECTIONS {
    /* GDT must be at 0x800 */
    .gdt 0x800 :
    {
        *(.gdt)
    }

    /* Kernel starts at 1MB */
    . = 1M;

    .boot ALIGN(4K) :
    {
        KEEP(*(.multiboot_header))
    }

    .text ALIGN(4K) :
    {
        *(.text)
    }

    .bss ALIGN(4K) :
    {
        *(.bss)
    }
}