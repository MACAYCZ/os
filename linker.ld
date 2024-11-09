ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS
{
	. = 1M;

	.text :
	{
		. = ALIGN(4);
		KEEP(*(.multiboot .multiboot.*))
		*(.text .text.*)
	}

	.rodata :
	{
		*(.rodata .rodata.*)
	}

	.data :
	{
		*(.data .data.*)
	}

	.bss :
	{
		*(.bss .bss.*)
	}
}
