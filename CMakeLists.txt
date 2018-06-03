cmake_minimum_required(VERSION 3.10)

add_subdirectory(kernel)

set(BUILD_TARGET x86_64-example_os)
set(KERNEL_BINARY "kernel.bin")

set(BOOTABLE_ISO ${BUILD_TARGET}.iso)
set(ISO_FILES_DIR ${CMAKE_CURRENT_BINARY_DIR}/iso_files)
set(GRUB_CONFIG ${CMAKE_CURRENT_SOURCE_DIR}/grub.cfg)
set(GRUB_BOOT_TOOL /usr/local/bin/grub-mkrescue)

set(TAR_TOOL tar)
set(METADATA_TOOL mat)
set(BOOT_DISK_ARCHIVE ${ISO_FILES_DIR}/boot/boot_disk.tar)
set(BOOT_DISK ${CMAKE_CURRENT_SOURCE_DIR}/boot_disk)

set(SYMBOL_STRIP_TOOL nm)
set(OBJECT_COPY_TOOL gobjcopy)
set(TEXT_ONLY_BINARY ${CMAKE_CURRENT_BINARY_DIR}/kernel/stripped_kernel.bin)
set(SYMBOL_TABLE ${BOOT_DISK}/kernel/symbols.table)

add_custom_target(${BOOTABLE_ISO})
add_dependencies(${BOOTABLE_ISO} ${KERNEL_BINARY})
configure_file(${GRUB_CONFIG} ${ISO_FILES_DIR}/boot/grub/grub.cfg)
add_custom_command(TARGET ${BOOTABLE_ISO} PRE_BUILD
		COMMAND ${CMAKE_COMMAND} -E make_directory ${ISO_FILES_DIR}/boot
		COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_CURRENT_SOURCE_DIR}/boot_disk/kernel

		COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:${KERNEL_BINARY}> ${ISO_FILES_DIR}/boot/kernel.bin

		COMMAND ${CMAKE_COMMAND} -E remove ${TEXT_ONLY_BINARY}
		COMMAND ${OBJECT_COPY_TOOL} --only-section=.text $<TARGET_FILE:${KERNEL_BINARY}> ${TEXT_ONLY_BINARY} 2> /dev/null
		COMMAND ${SYMBOL_STRIP_TOOL} ${TEXT_ONLY_BINARY} > ${SYMBOL_TABLE}

		COMMAND ${TAR_TOOL} cf ${BOOT_DISK_ARCHIVE} -C ${BOOT_DISK} .
		COMMAND ${GRUB_BOOT_TOOL} -o ${CMAKE_CURRENT_BINARY_DIR}/${BOOTABLE_ISO} ${ISO_FILES_DIR} 2> /dev/null
		COMMAND ${CMAKE_COMMAND} -E remove_directory ${ISO_FILES_DIR})
