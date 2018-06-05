---
title: Entering user mode in long mode
description: Implementation details on entering the user mode privilege level from long mode
tags: user-mode descriptors global-descriptor-table interrupts interrupt-stack-frame
---
Note: This post does not discuss context switching, but the actual
method of which to get into user mode.

## What?
There's four different rings in the Intel processor. Ring 0 is called
kernel mode and Ring 3 is called user mode. User mode has a lot more
security and restricted access to various instructions and areas of
memory.

## Why?
A rogue program running in Ring 0 can destroy the entire system, as it
has almost no bounds on what it can do. This is why programs
are ran in Ring 3; so that protection mechanisms can be put in place
and at the very least, limit the amount of damage a virus can do. It's
also to prevent buggy code from affecting the rest of the system.

## How?
The easiest way to enter Ring 3 is by hijacking an interrupt and
pretending we were in Ring 3 to begin with, before we entered the
interrupt. In exampleOS, we use the timer interrupt for this very
purpose in [this file](https://github.com/Techno-coder/example_os/blob/e98b970e16662589d2b05b98f5b4990f9ac5a789/kernel/src/interrupts/handlers.rs#L44).

## The Global Descriptor Table
Before paging was introduced, the main memory protection mechanism was
through the use of "segmentation". Essentially, regions of memory would
be assigned a descriptor which describes the properties of that region, such
as the privilege level to access it, whether it is executable, and whether it is
writable. In long mode, we no longer use segmentation for protection, but it
is still required. Instead, we just create two segments, for code and for data,
and have it span the entire address space.

Descriptors can be marked as either Ring 0 or Ring 3. More importantly,
they tell the processor what privilege mode we were in before entering
an interrupt. We will use this later to jump into user mode.

## The Interrupt Stack Frame
When an interrupt is received by the processor, two important things happen.
1. The processor saves some data about its current state onto the stack
2. The processor jumps into the appropriate interrupt handler  

The data that is saved is called the Interrupt or Exception Stack Frame.
Here's what it looks like:

<img src="{{site.baseurl}}/assets/user_mode/exception_frame.png" style="width: 300px;">

The two important elements here are the stack and code segment. If their descriptors'
privilege level is Ring 3 then the processor thinks that we were in Ring 3
before the interrupt occurred. More importantly, when we return from the
interrupt, the processor will place us into Ring 3.

## Creating the descriptors
You'll need to add two new descriptors to your GDT with these flags:  
User Code Segment: `USER_SEGMENT | PRESENT | EXECUTABLE | LONG_MODE | RING_USER`  
User Data Segment: `USER_SEGMENT | PRESENT | WRITABLE	| LONG_MODE	| RING_USER`  

The hexadecimal version of these descriptors are:  
User Code Segment: `0x0020_f800_0000_0000`  
User Data Segment: `0x0020_f200_0000_0000`  

These descriptors are almost the same as the kernel code and data descriptors except
they have the `RING_USER` flag enabled.
See exampleOS's construction of these descriptors [here](https://github.com/Techno-coder/example_os/blob/master/kernel/src/interrupts/gdt_descriptor.rs).

## Referencing the descriptors
Once you've added the new descriptors to the table, you'll need their
segment selectors as well.

Segment selectors act as an index into the Global Descriptor Table, as they
point to a specific descriptor. They are 16 bits long. Bits 3 to 15 store the
actual index into the GDT and bits 0 to 1 store the "requested privilege level" (or RPL for short).
For a program to access a segment, their current descriptor's RPL needs to be
lower or equal to the current privilege level of the processor. For our purposes,
all we need the RPL to be is Ring 3.

The index of a descriptor is calculated by their byte index divided by eight (because 
descriptors have a size of eight bytes). For example, if you already had three descriptors
in the GDT (including the null descriptor), the fourth descriptor would have an index of 4.

exampleOS's segment selectors can be found [here](https://github.com/Techno-coder/example_os/blob/e98b970e16662589d2b05b98f5b4990f9ac5a789/kernel/src/interrupts/functions.rs#L53).

## The FLAGS register
The FLAGS register stores information about the state of the processor. For now,
we don't need to worry about it, but we do need to create a valid FLAGS value. The
most minimal FLAGS register value is: `0x2` which only has the flag `RESERVED` set.

## Faking the stack
Now we can try and get into user mode. First you need to setup an interrupt stack frame:
```asm
pushq USER_DATA_SEGMENT_SELECTOR
pushq rsp
pushq 0x2
pushq USER_CODE_SEGMENT_SELECTOR
pushq ADDRESS_OF_FUNCTION
```
Replace `USER_DATA_SEGMENT_SELECTOR` and `USER_CODE_SEGMENT_SELECTOR` with their numerical
values. Replace `ADDRESS_OF_FUNCTION` with the address of a function you want executed
in user mode. See exampleOS's faked stack [here](https://github.com/Techno-coder/example_os/blob/e98b970e16662589d2b05b98f5b4990f9ac5a789/kernel/src/task/loaders/stack.rs#L18).

Finally, add this instruction at the end:  
```asm
iret
```
This is the instruction for returning from an interrupt. The processor will pop off all
the pushed values, and hopefully bring us into user mode.

## Now what?
If you're lucky, you should have either gotten a page fault or nothing happened at all.
If you get a General Protection Fault, double check that all your descriptors are loaded
properly, and the selectors are all correct. The address of the function should not be zero
either as executing code at address zero will cause a fault.

### Interrupts are not working anymore
If you've had a timer or keyboard interrupt set up, you may notice that it's no longer firing. This is because
for interrupts to work, the `INTERRUPT_ENABLE` flag needs to be set in the FLAGS register. If you want interrupts,
this is the FLAGS value to push: `0x202`

### Fixing the page fault
The page that the function resides in must be marked as `USER_ACCESSIBLE` and `EXECUTABLE`.
Additionally, the page directories needed to access the page **must also be marked** as
`USER_ACCESSIBLE`. This is because when the processor resolves a virtual address, it needs 
to read all the page directories to access the page. exampleOS always adds the `USER_ACCESSIBLE`
flag when [creating a page directory](https://github.com/Techno-coder/example_os/blob/e98b970e16662589d2b05b98f5b4990f9ac5a789/kernel/src/paging/page_table.rs#L41).

## Caveat
We push the `rsp` register onto the exception stack frame. This is bad for two reasons:
1. The stack is located in a kernel page
2. The user mode program shares the same stack with the kernel

If you call any function, then a page fault will occur because the processor can't write to
the stack. Typically, you should have a separate stack for every user mode program. In a later
post, we will fix this issue and look at how a user mode program is organised in memory.

[Back to the root](https://techno-coder.github.io/example_os/)
