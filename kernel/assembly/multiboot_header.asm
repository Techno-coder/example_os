MULTIBOOT_MAGIC_NUMBER: equ 0xe85250d6
ARCHITECTURE_i386:		equ 0
HEADER_LENGTH: 			equ header_end - header_start
MULTIBOOT_CHECKSUM:		equ 0x100000000 - (MULTIBOOT_MAGIC_NUMBER + HEADER_LENGTH)

struc MultibootTagHeader
.type:	resw 1
.flags: resw 1
.size:	resd 1
endstruc

section .multiboot_header
header_start:
    dd MULTIBOOT_MAGIC_NUMBER
    dd ARCHITECTURE_i386
    dd HEADER_LENGTH

    dd MULTIBOOT_CHECKSUM

end_tag: istruc MultibootTagHeader
	at MultibootTagHeader.type,		dw 0
	at MultibootTagHeader.flags,	dw 0
	at MultibootTagHeader.size,		dd 8
iend

header_end:
