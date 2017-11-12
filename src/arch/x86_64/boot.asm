;entry point of kernel
global start


;default section for executable code
section .text

;when the GRUB starts the kernel, CPU uses protected mode, which is 16 bits 
;following is for 32-bit instructions
bits 32
start:
    ;prints 'OK' to screen
    mov dword[0xb8000], 0x2f4b2f4f
    hlt
