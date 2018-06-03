%include "constants.inc"

global start

extern boot_entry
extern check_multiboot
extern check_cpuid
extern check_long_mode
extern setup_page_tables
extern enable_paging
extern invalidate_tables
extern pml4_table
extern boot_stack_top

section .boot_entry
bits 32
start:
    mov esp, boot_stack_top - KERNEL_BASE  ; set up stack
    mov edi, ebx

    call check_multiboot
    call check_cpuid
    call check_long_mode
    call setup_page_tables
    call enable_paging

    lgdt [gdt64.pointer_low - KERNEL_BASE]

    ; update selectors
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax

    jmp gdt64.code:prestart64

bits 64
prestart64:
    mov rax, start64
    jmp rax

section .text
start64:
    mov rsp, boot_stack_top
    mov rax, gdt64.pointer
    lgdt [rax]

    ; update selectors
    mov ax, 0
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

    jmp start64_2

start64_2:
    mov rax, pml4_table
    mov qword [rax], 0
    invlpg [0]
.create_null_frame:
	xor rbp, rbp
    push rbp
.entry:
    call boot_entry

section .rodata
align 8
gdt64:
    dq 0                                                ; zero entry
.code: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)  ; code segment
.data: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41)                      ; data segment
.end:
.pointer:
    dw gdt64.end - gdt64 - 1
    dq gdt64
.pointer_low:
    dw gdt64.end - gdt64 - 1
    dq gdt64 - KERNEL_BASE