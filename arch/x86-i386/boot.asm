global start
extern load_gdt
extern _start

section .text
bits 32
start:
    mov esp, stack_top      ; Initialize stack pointer
    call load_gdt           ; Load our GDT
    call _start            ; Jump to Rust code
    hlt                    ; Halt CPU if we return

section .bss
align 4096
stack_bottom:
    resb 4096 * 256        ; 1MB stack size
stack_top: