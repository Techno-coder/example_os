%include "constants.inc"

global setup_page_tables
global enable_paging
global invalidate_tables
global pml4_table
global boot_stack_top

section .boot_entry
bits 32
setup_page_tables:
    ; map first and 510th PML4 entries to PDP table
    mov eax, pdp_table - KERNEL_BASE
    or eax, 0b11    ; present + writable
    mov [pml4_table - KERNEL_BASE], eax
    mov [pml4_table - KERNEL_BASE + 510 * 8], eax
    ; map first PDP entry to PD table
    mov eax, pd_table - KERNEL_BASE
    or eax, 0b11    ; present + writable
    mov [pdp_table - KERNEL_BASE], eax
    ; map each PD entry to a huge (2MiB) page
    mov ecx, 0
.next_pd_entry:
    mov eax, 0x200000
    mul ecx
    or eax, 0b10000011  ; present + writable + huge
    mov [pd_table - KERNEL_BASE + ecx * 8], eax
    inc ecx
    cmp ecx, 512
    jne .next_pd_entry
.recursive_pml4_table:
	mov eax, pml4_table - KERNEL_BASE
    or eax, 0b11 ; present + writable
    mov [pml4_table - KERNEL_BASE + 511 * 8], eax
    ret

enable_paging:
    ; load PML4 to cr3 register
    mov eax, pml4_table - KERNEL_BASE
    mov cr3, eax

    ; enable PAE flag in cr4
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit in the EFER MSR
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    ret

section .bss
align 4096
pml4_table:
    resb 4096
pdp_table:
    resb 4096
pd_table:
    resb 4096
page_table:
    resb 4096
boot_stack_bottom:
    resb 16 * 4096
boot_stack_top:
