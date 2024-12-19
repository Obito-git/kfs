global start
global stack_end
global stack_start
extern load_gdt
extern _start

section .text
bits 32
start:
    mov esp, stack_start    ; Initialize stack pointer to the top of the stack
    call load_gdt           ; Load the Global Descriptor Table (GDT)
    call _start             ; Jump to Rust code
    hlt                     ; Halt CPU if we return

section .bss
align 4096
stack_end:
    resb 4096 * 256         ; Reserve 1MB for the stack
stack_start:                ; Top of the stack (last byte of reserved memory)
