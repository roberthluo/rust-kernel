/*entry point from bootloader*/
ENTRY(start)

SECTIONS {
    /*sets the load address of first section to 1 MiB*/ 
    . = 1M;
    
    /*boot section*/
    .boot :
    {
        /* ensure that multiboot header is at beginning*/
        *(.multiboot_header)
    }
    /* contains all input section name .text */
    .text :
    {
        *(.text)
    }
}
