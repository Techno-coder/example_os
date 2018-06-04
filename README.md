# example_os
A heavily commented OS in Rust for reference purposes (documentation in progress).  
Check out the companion site here: [Tutorials](https://techno-coder.github.io/example_os/)

This OS is a hard fork of my private OS that I'm also working on. No more features will be added to this OS as it is for reference purposes only.  
Huge thanks to [Phil-opp](https://os.phil-opp.com) and the [OSDev wiki](https://wiki.osdev.org)

## License
Everything in this repository is under the MIT license, excluding the tutorials folder, (which contains data for the companion website) which is copyrighted. 

## Features
- Preemptive Multitasking (including system calls)
- System Calls (well, one of them)
- Primitive filesystem (Tar archive ram disk)
- Stack traces (with kernel symbols)
- Huge page support
- Huge frame allocation
- Keyboard driver
- Kernel shell (with tab completion)

## Prerequisites
You can find most of the prerequisites needed in the `CMakeLists.txt` files located in the root and in `kernel` folder.

Build requirements:
- NASM Assembler
- Binutils
- nm (Symbol table displayer)
- Rust
- Cargo
- Xargo
- LLVM linker
- tar (for creating the boot disk)
- grub-mkrescue (to generate the .iso file)
- CMake (if you wish to compile with CMake)

Execution requirements:
- QEMU
- Bochs

A `bochsrc` file is located in the root.

## Build instructions
Once you have installed the prerequisites, you can either compile everything manually or use CMake.

### CMake
Change `XARGO_TOOL` in the kernel CMake file from `xargo-m` to `xargo`. The -m suffix is to avoid Intellij-Rust from using Xargo to run tests.  
Change `OBJECT_COPY_TOOL` from `gobjcopy` to `objcopy` or whatever the `objcopy` tool is aliased to.

Finally, run this command:
`cmake --build cmake-build-debug --target x86_64-example_os.iso`  
The .iso file will be located in the cmake-build-debug folder

### CLion
Import the project and then build the `.iso` target after making the changes in the CMake section.

### Manually
1. Use NASM to compile all the files in `kernel/assembly` with the command  
`nasm -felf64 -w-number-overflow -Ikernel/assembly`  
2. Use Xargo to compile the Rust binary with the command (execute in the `kernel` folder)  
``RUST_FLAGS=-Cforce-frame-pointers=yes RUST_TARGET_PATH=`pwd` xargo build --target x86_64-example_os``  
The `RUST_TARGET_PATH` is needed to allow xargo to locate the target specification.
3. Link all the files together  
`ld.lld --gc-sections -Tlinker.ld <Assembly object files> <Generated Rust file>`  
The Rust file is located in `kernel/target/x86_64-example_os/debug/libkernel.a`  
4. Create a `boot_disk` folder, and a `kernel` folder inside of that
5. Use `nm` against the linked binary and pipe it to a file `symbols.table`.  
Copy this file into the `boot_disk/kernel` folder.
6. Tar the `boot_disk` folder into `boot_disk.tar`
7. Create a `iso_files` folder, and a `boot` folder inside of that, and a `grub` folder inside of that
8. Copy the `grub.cfg` file into `iso_files/boot/grub`
9. Copy the `boot_disk.tar` into `iso_files/boot`
10. Create the .iso with the command  
`grub-mkrescue -o <ISO file> iso_files`

## Execution
### Bochs
Just run `bochs` in the root directory. You may need to change the path to the iso file in the `bochsrc`.
### QEMU
Run the command  
`qemu-system-x86_64 -cdrom x86_64-example_os.iso -monitor stdio -d int -no-reboot -no-shutdown`
