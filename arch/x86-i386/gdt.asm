section .gdt           ; Special GDT section that will be placed at 0x800
align 8
global gdt_ptr
global gdt_start
global gdt_end

gdt_ptr:
    dw (gdt_end - gdt_start - 1)   ; GDT size minus 1
    dd gdt_start                    ; GDT start address

gdt_start:
    ; Null descriptor (required)
    dd 0x0
    dd 0x0

    ; Kernel Code segment
    dw 0xFFFF    ; Limit
    dw 0x0       ; Base
    db 0x0       ; Base
    db 0x9A      ; Access - Ring 0, Code
    db 0xCF      ; Flags + Limit
    db 0x0       ; Base

    ; Kernel Data segment
    dw 0xFFFF
    dw 0x0
    db 0x0
    db 0x92      ; Access - Ring 0, Data
    db 0xCF
    db 0x0

    ; Kernel Stack segment
    dw 0xFFFF
    dw 0x0
    db 0x0
    db 0x92      ; Access - Ring 0, Data
    db 0xCF
    db 0x0

    ; User Code segment
    dw 0xFFFF
    dw 0x0
    db 0x0
    db 0xFA      ; Access - Ring 3, Code
    db 0xCF
    db 0x0

    ; User Data segment
    dw 0xFFFF
    dw 0x0
    db 0x0
    db 0xF2      ; Access - Ring 3, Data
    db 0xCF
    db 0x0

    ; TODO: delete after KFS-2
    ; User Stack segment
    dw 0xFFFF
    dw 0x0
    db 0x0
    db 0xF2      ; Access - Ring 3, Data
    db 0xCF
    db 0x0
gdt_end:

section .text
global load_gdt
bits 32

load_gdt:
    lgdt [gdt_ptr]          ; Load GDT register

    mov ax, 0x10           ; 0x10 is kernel data segment
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    jmp 0x08:.reload_cs    ; 0x08 is kernel code segment
.reload_cs:
    ret