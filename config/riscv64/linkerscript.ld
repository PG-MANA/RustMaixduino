MEMORY {
  ram_with_cache : ORIGIN = 0x80000000, LENGTH = (6 * 1024 * 1024)
  ram_without_cache : ORIGIN = 0x40000000, LENGTH = (6 * 1024 * 1024)
}

ENTRY(_start)

SECTIONS {
    .text : ALIGN(8) {
        *(.text .text*)
    } > ram_with_cache AT> ram_with_cache

    .rodata : ALIGN(8) {
        *(.rodata .rodata.*)
    } > ram_with_cache AT> ram_with_cache

    .data : ALIGN(8) {
        *(.data .data.*)
        *(.sdata .sdata.*)
    } > ram_with_cache AT> ram_with_cache

    .bss : ALIGN(8) {
        *(.sbss .sbss.*)
        *(.bss .bss.*)
    } > ram_with_cache AT> ram_with_cache
}
