cmake_minimum_required(VERSION 3.8)
enable_language(ASM_NASM)

project(kernel)

set(BUILD_TARGET x86_64-example_os)
set(KERNEL_BINARY "${PROJECT_NAME}.bin")
set(LINKER_SCRIPT ${CMAKE_CURRENT_SOURCE_DIR}/linker.ld)

set(CMAKE_ASM_NASM_FLAGS "-felf64 -w-number-overflow -I${CMAKE_CURRENT_SOURCE_DIR}/assembly/")
set(CMAKE_ASM_NASM_COMPILE_OBJECT "<CMAKE_ASM_NASM_COMPILER> ${CMAKE_ASM_NASM_FLAGS} -o <OBJECT> <SOURCE>")
set(CMAKE_EXE_LINKER_FLAGS "--gc-sections -T${LINKER_SCRIPT}")
set(CMAKE_ASM_NASM_LINK_EXECUTABLE "ld.lld <LINK_FLAGS> -o <TARGET> <OBJECTS> <LINK_LIBRARIES>")

set(XARGO_TOOL $ENV{HOME}/.cargo/bin/xargo-m)
set(RUST_PROJECT ${PROJECT_NAME}.a)
set(RUST_OUTPUT_PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/target/${BUILD_TARGET})

# force-frame-pointers=yes is needed to allow stack traces to work properly
# See src/debug/stack_trace
set(COMMAND_PREFIX RUST_TARGET_PATH=${CMAKE_CURRENT_SOURCE_DIR} RUSTFLAGS=-Cforce-frame-pointers=yes)
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
	set(RUST_BUILD ${RUST_OUTPUT_PREFIX}/debug/lib${PROJECT_NAME}.a)
else ()
	set(RUST_BUILD ${RUST_OUTPUT_PREFIX}/release/lib${PROJECT_NAME}.a)
endif ()

file(GLOB_RECURSE ASSEMBLY_SOURCES assembly/*.asm)

add_custom_target(${RUST_PROJECT})
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
	add_custom_command(TARGET ${RUST_PROJECT} PRE_BUILD
			COMMAND ${COMMAND_PREFIX} ${XARGO_TOOL} build --target ${BUILD_TARGET}
			WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
else ()
	add_custom_command(TARGET ${RUST_PROJECT} PRE_BUILD
			COMMAND ${COMMAND_PREFIX} ${XARGO_TOOL} build --release --target ${BUILD_TARGET}
			WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
endif ()

add_executable(${KERNEL_BINARY} ${ASSEMBLY_SOURCES})
set_target_properties(${KERNEL_BINARY} PROPERTIES LINK_DEPENDS ${LINKER_SCRIPT})
add_dependencies(${KERNEL_BINARY} ${RUST_PROJECT})
target_link_libraries(${KERNEL_BINARY} ${RUST_BUILD})
