	.section .text.entry
	.globl _start
_start:
	la sp, boot_stack_base
	call rust_init_entry

	.section .bss.stack
	.globl boot_stack
boot_stack:
	.space 4096 * 16
	.globl boot_stack_base
boot_stack_base:
