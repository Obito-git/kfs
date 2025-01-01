section .data
align 4
global multiboot_info_addr   ; Declare the symbol as global
multiboot_info_addr:
    dd 0                     ; Reserve 4 bytes, initialize to 0

global _start
global stack_end
global stack_start
extern load_gdt
extern kmain

section .text
bits 32
_start:
    mov esp, stack_start            ; Initialize stack pointer to the top of the stack
    mov [multiboot_info_addr], ebx  ; Save multiboot info address to global variable
    call load_gdt                   ; Load the Global Descriptor Table (GDT)
    call kmain                      ; Jump to Rust code
    hlt                             ; Halt CPU if we return

section .bss
align 4096
stack_end:
    resb 4096 * 256         ; Reserve 1MB for the stack
stack_start:                ; Top of the stack (last byte of reserved memory)
