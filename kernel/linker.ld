ENTRY(start)

KERNEL_BASE = 0xFFFFFF0000000000;

SECTIONS {
	# Set the current address to the lower half
    . = 1M;

    .boot_header :
    {
        KEEP(*(.multiboot_header))
        . = ALIGN(4K);
    }

    .boot_entry : {
        *(.boot_entry)
        . = ALIGN(4K);
    }

	# All sections after this have addresses
	# in the higher half
    . += KERNEL_BASE;

	.rodata : AT(ADDR(.rodata) - KERNEL_BASE) {
      *(.rodata .rodata.*)
      *(.eh_frame)
      . = ALIGN(4K);
    }

    .text : AT(ADDR(.text) - KERNEL_BASE) {
      *(.text .text.*)
      . = ALIGN(4K);
    }

    .data : AT(ADDR(.data) - KERNEL_BASE) {
      *(.data .data.*)
      . = ALIGN(4K);
    }

    .bss : AT(ADDR(.bss) - KERNEL_BASE) {
      *(.bss .bss.*)
      . = ALIGN(4K);
    }

    .got : AT(ADDR(.got) - KERNEL_BASE) {
      *(.got)
      . = ALIGN(4K);
    }

    .got.plt : AT(ADDR(.got.plt) - KERNEL_BASE) {
      *(.got.plt)
      . = ALIGN(4K);
    }

    .gcc_except_table : ALIGN(4K) {
      *(.gcc_except_table)
      . = ALIGN(4K);
    }
}