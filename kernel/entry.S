	.code32

	.section .multiboot, "a"
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
	cld
	mov esp, offset stack
	xor ebp, ebp

	cli
	lgdt gdt
	ljmp 0x08, offset _start.next

_start.next:
	mov cx, 0x10
	mov ds, cx
	mov ss, cx
	mov es, cx
	mov fs, cx
	mov gs, cx

	call main

	.size _start, . - _start

	.section .rodata
gdt.null:
	.long 0x00
	.long 0x00

gdt.code:
	.word 0xFFFF
	.word 0x00
	.byte 0x00
	.byte 0x9A
	.byte 0xCF
	.byte 0x00

gdt.data:
	.word 0xFFFF
	.word 0x00
	.byte 0x00
	.byte 0x92
	.byte 0xCF
	.byte 0x00

gdt:
	.word . - gdt.null - 1
	.long gdt.null

	.bss
	.align 16
	.space 1024*1024
stack:
