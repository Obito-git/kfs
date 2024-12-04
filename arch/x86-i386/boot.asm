global start

section .text
bits 32
start:
    extern _start
    call _start

    mov eax, 0x2f592f42   ; 'BY' with attributes
    mov dword [0xb8000], eax
    mov eax, 0x2f452f45   ; 'EE' with attributes (second E is padding)
    mov dword [0xb8004], eax
    hlt