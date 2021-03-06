global syscall_handler
extern syscall_handler_inner

section .text
bits 32
syscall_handler:
    push eax
    push ebx
    push ecx
    push edx
    push esi
    push edi

    mov eax, esp

    push ebp

    ; Data segments
    push gs
    push fs
    push es
    push ds

    push eax

    mov ax, 0x10
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov es, ax

    call syscall_handler_inner
    
    add esp, 4

    ; Data segments
    pop ds
    pop es
    pop fs
    pop gs

    pop ebp

    pop edi
    pop esi
    pop edx
    pop ecx
    pop ebx
    ; Dont pop eax
    add esp, 4

    iretd

