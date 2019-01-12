# rust-kernel
Learning how a kernel works 32bit on x86 architecture

Links:
https://doc.rust-lang.org/book/first-edition

Partially based off:
https://github.com/jvns/puddle
https://os.phil-opp.com/multiboot-kernel/


installation instructions:

```bash
$ dnf install nasm
$ dnf install qemu
```
Differences:
```
Using grub2-mkrescue
```
When a computer is turned on, it loads some BIOS, and the control is transfered to a bootloader. There is a standard for a bootloader called the Multiboot Specification.


## What does it do?
It prints to screen using the CGA text buffer.
