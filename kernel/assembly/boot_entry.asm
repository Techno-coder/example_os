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

; This section is NOT in the higher half
section .boot_entry
bits 32
start:
    mov esp, boot_stack_top - KERNEL_BASE  ; set up stack
    mov edi, ebx

	; See boot_checks.asm
    call check_multiboot
    call check_cpuid
    call check_long_mode

    ; See boot_paging.asm
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
	; Jump to the higher half
    mov rax, start64
    jmp rax

; This section is in the higher half
; See linker.ld
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
	; We don't need any pages mapped in the
	; lower half anymore, so we unmap them
	; and invalidate the entry
    mov rax, pml4_table
    mov qword [rax], 0
    invlpg [0]
.create_null_frame:
	; Set the base pointer to null
	xor rbp, rbp
    push rbp
.entry:
	; Enter rust code
    call boot_entry

section .rodata
align 8
gdt64:
    dq 0                                                ; zero entry
.code: equ $ - gdt64
	; Reserved, Present, Readable, Executable, Long Mode
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)  ; code segment
.data: equ $ - gdt64
	; Reserved, Present, Writable
    dq (1<<44) | (1<<47) | (1<<41)                      ; data segment
.end:
.pointer:
    dw gdt64.end - gdt64 - 1
    dq gdt64
.pointer_low:
    dw gdt64.end - gdt64 - 1
    dq gdt64 - KERNEL_BASE