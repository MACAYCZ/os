	.code32

	.section .multiboot
multiboot:
	.set multiboot.magic, 0x1badb002
	.set multiboot.flags, 0x00

	.long multiboot.magic
	.long multiboot.flags
	.long -(multiboot.magic + multiboot.flags)

	.text
	.globl _start
	.type _start, @function
_start:
	hlt
