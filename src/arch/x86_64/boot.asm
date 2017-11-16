;entry point of kernel
global start


;default section for executable code
section .text

;when the GRUB starts the kernel, CPU uses protected mode, which is 16 bits 
;following is for 32-bit instructions
bits 32
start:
    ;stack pointer register pointing to top 
    mov esp, stack_top
    ;prints 'OK' to screen
    mov dword[0xb8000], 0x2f4b2f4f
    hlt

; Prints 'ERR:' amd the given error code to secreen and hangs.
; paramter: error code in asciii
error:
    move dword [0xb8000], 0x4f524f45
    move dword [0xb8004], 0x4f3a4f52
    move dword [0xb8008], 0x4f204f20
    mov byte   [0xb800a], al
    hlt


; Creating stack memory by reserving some bytes
section .bss
stack_bottom:
    resb 64
stack_top:
