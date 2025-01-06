# `sim8086`
**sim8086** is a lightweight simulator for the 8086 family of Intel processors, built to decode raw machine code into 8086 Assembly and emulate execution as it would occur on real hardware.

This project is primarily educational: to write efficient software, it helps to understand how the CPU operates at a low level. 
By reading and analyzing assembly, you can spot any inefficiencies or “waste” in your program. 
sim8086 provides an introduction to these concepts, and the accompanying
[instruction manual](https://edge.edx.org/c4x/BITSPilani/EEE231/asset/8086_family_Users_Manual_1_.pdf)
serves as a handy reference.

Although sim8086 does not yet implement every single 8086 instruction, it covers most of them well enough to handle typical programs of moderate complexity. 
I encourage you to consult the
[instruction manual](https://edge.edx.org/c4x/BITSPilani/EEE231/asset/8086_family_Users_Manual_1_.pdf)
and consider contributing to improve and extend the simulator.
For all instruction for which decoding and execution has been implemented, there are accompanying tests in `sim8086/tests/`.
To run the decoding tests you must have [NASM](https://www.nasm.us/) installed. 
Additionally, there is a test file that contains all available instructions `sim8086/tests/decode/test_all`, and currently fails, 
because of runtime errors which will signify which instructions have not been implemented yet.

## Installation

To install sim8086 clone this repository and build it with `cargo` either in `debug` or `release` mode.
```
git clone https://github.com/gsmyridis/sim8086.git
cd sim8086
cargo build --release
```
The binary is then located in `target/debug/sim8086` or `target/release/sim8086` depending on the build mode.

## Usage

sim8086 has two main commands: `decode` and `execute`.

### Decoding

The `decode` command reads a byte-code file (raw 8086 machine code) and produces an Assembly listing.
```
sim8086 decode <OUTPUT> <INPUT>
```
- `<OUTPUT>`: Path to the file where the resulting Assembly code will be written.
- `<INPUT>`: The path to the binary file containing the 8086 machine code.

### Executing

The `execute` command runs the 8086 machine code under the simulator, allowing you to see the program’s behavior.
In `STDOUT` you will see the 8086 Assembly code along with the final state of the CPU.
```
sim8086 execute <INPUT>
```
- `<INPUT>`: The path to the binary file containing the 8086 machine code to simulate.

`sim8086` cannot execute Assembly listings, it can only decode byte-code into intermediate representations which are then executed.


# 8086 Architecture Overview

In this section, we briefly describe the relevant parts of the 8086 processor architecture and its operation that are necessary for our simulator and for developing a robust model. 
These elements are based on the instruction manual, in some cases quoted directly.

## Processor Architecture

The 8086 are third-generation processors.
Microprocessors generally execute a program by repeatedly cycling through the simplified steps below:

1. Fetch the next instruction from memory.
2. Read an operand (if required by the instruction).
3. Execute the instruction.
4. Write the result (if required by the instruction).

In CPUs prior to 8086, most of these steps have been performed serially, or with only a single bus cycle fetch overlap.
The architecture of the 8086, while performing the same steps, allocates them to two separate processing units within the CPU.
The execution unit (EU) executes instructions, and the bus interface unit (BUI) fetches instructions, reads operands and writes results.
*In our simulator there is no distinction between the two units.*

### Execution Unit

A 16-bit arithmetic/logic unit (ALU) in the EU maintains the CPU status and control flags, and manipulates the general registers and instruction operands.
All registers and data paths in the EU are 16 bits wide for fast internal transfers.

The EU has no connection on the system bus. It obtains instructions from a queue maintained by the BIU.
Likewise, when an instruction requires access to memory or to a peripheral device, the EU requests the BIU to obtain or store the data.
All addresses manipulated by the EU are 16 bits wide.
The BIU, however, performs an address relocation that gives the EU access to the full megabyte of memory space.

### Bus Interface Unit

The BIU performs all bus operations for the EU.
Data is transferred between the CPU and memory or I/O devices upon demand from the EU.

### General Registers

Registers are subdivided into two sets of four registers each: the data registers (sometimes called the H&L group for "high" and "low"), and the pointer and index registers (sometimes called the P&I group).

The data registers are unique in that their upper (high) and lower halves are separately addressable.
This means that each data register can be used interchangeably as a 16-bit register, or as two 8-bit registers.
The other CPU registers always are accessed as 16-bit units only.
The data registers can be used without constraint in most arithmetic and logic operations.
In addition, some instructions use certain registers implicitly thus allowing compact yet powerful encoding.

The pointer and index registers can also participate in most arithmetic and logic operations.
The P&I registers, except BP also are used implicitly in some instructions.

### Segment Registers

The megabyte of 8086 memory space is divided into logical segments of up to 64kB each.
The CPU has direct access to four segments at a time; their base addresses (starting locations) are contained in the segment registers.

- The `CS` register points to the current code segment; instructions are fetched from this segment.
- The `SS` register points to the current stack segment; stack operations are performed on locations in this segment.
- The `DS` register points to the current data segment; it generally contains program variables.
- The `ES` register points to the current extra segment, which also iis typically used for data storage.

The segment registers are accessible to programs and can be manipulated with several instructions.

### Instruction Pointer

The instruction pointer is updated by the BIU so that it contains the offset (distance in bytes) of the next instruction from the beginning of the current code segment.
That is to say that `IP` points to the next instruction.
During normal execution, `IP` contains the offset of the next instruction to be **fetched**  by the BIU.
However, whenever the `IP` is saved on the stack, it first is automatically adjusted to point to the next instruction to be **executed**.
Programs do not have direct access to the instruction pointer, but instructions cause it to change and to be saved on and restored from the stack.

### Flags

There are size 1-bit status flags that the EU posts to reflect certain properties of the result of an arithmetic or logic operation.
A group of instructions is available that allows a program to alter its execution depending on the state of these flags, that is, on the result of the prior operation.
Different instructions affect the status flags differently; in general, however, the flags reflect the following conditions:

1. If `AF`, the auxiliary carry flag, is set, there has been a carry out of the low nibble into the high nibble or a borrow from the high nibble into the low nibble of an 8-bit quantity (low-order byte of a 16-bit quantity). This flag is used by decimal arithmetic instructions.
2. If `CF`, the carry flag, is set, there has been a carry out of, or a borrow into, the high-order bit of the result (8- or 16-bit). The flag is used by instructions that add and subtract multibyte numbers. Rotate instructions can also isolate a bit in memory or a register by placing it in the carry flag.
3. If `OF`, the overflow flag, is set, an arithmetic overflow has occurred; that is, a significant digit has been lost because the size of the result exceeded the capacity of its destination location. An Interrupt On Overflow instruction is available that will generate an interrupt in this situation.
4. If `SF`, the sign flag, is set, the high-order bit of the result is 1. Since negative binary numbers are represented in standard's two notation, `SF` indicates the sign of the result. 0 is for positive, and 1 for negative.
5. If `PF`, the parity flag, is set, the result is an even number.
6. If `ZF`, the zero flag, is set, the result is zero.

Three additional control flags can be set and cleared by programs to alter processor operations:

1. Setting `DF`, the direction flag causes string instructions to auto-decrement; that is, to process strings from high address to low address, or from "right to left".
2. Setting `IF`, the interrupt-enable flag, allows the CPU to recognize external (maskable) interrupts. Clearing `IF` disables these interrupts.
3. Setting `TF`, the trap flag, puts the processor into single-step mode for debugging. In this mode, the CPU automatically generates an internal interrupt after each instruction, allowing a program to be inspected as it executes instruction by instruction.

## Memory

The 8086 can accommodate up to 1MB of memory. *However, in our simulator this is impossible because its stack overflows*.

### Storage Organisation

From a storage point of view, the 8086 memory spaces are organized as arrays of bytes. 
This means that the smallest addressable unit of memory is a single byte; each byte has its own address.
Instructions, byte data and word data may be freely stored at any byte address without regard for alignment, thereby saving memory space by allowing code to be densely packed in memory.
Odd-addressed (unaligned) word variables, however, do not take advantage of the 8086's ability to transfer 16-bits at a time.
Instruction alignment does not affect the performance of the processor.

Following Intel convention, word data always is stored with the most-significant byte in the higher memory location. 
In other words, the 8086 processor is little-endian.

### Segmentation

8086 programs view the megabyte of memory space as a group of segments that are defined by the application.
A segment is a logical unit of memory that may be up to 64kB long.
Each segment is made up of contiguous memory locations and is an independent, separately addressable unit.
Every segment is assigned (by software) a base address, which is its starting location in the memory space. 
All segments begin on 16-byte memory boundaries.
There are no other restrictions on segment locations; segments may be adjacent, disjoint, partially overlapped, or fully overlapped.
A physical memory location may be mapped into one or more logical segments.

Every application will define and use segments differently.
The currently addressable segments provide a workspace of 64kB for code, 64kB stack, and 128 kB of data storage.

### Physical Address Generation

It is useful to think of every memory location as having two kinds of addresses, physical and logical.
A physical address is the 20-bit value that uniquely identifies each byte location in the megabyte memory space.
Physical addresses may range from 0x0H through 0xFFFFFH. 
All exchanges between the CPU and memory components use this physical address.

Programs deal with logical, rather than physical addresses and allow code to be developed without prior knowledge of where the code is to be located in memory and facilitate dynamic management of memory resources.
A logical address consists of a segment base value and an offset value. 
For any given memory location, the segment base value locates the first byte of the containing segment and the offset value is the distance, in bytes, of the target location from the beginning of the segment.
Many different logical addresses can map to the same physical location.

Whenever the BIU accesses memory -- to fetch an instruction or to obtain or store a variable -- it generates a physical address from a logical address.
This is done by shifting the segment base value four bit positions and adding the offset. 
*This has not yet been implemented for our simulator which currently works with 64kB long memory.*
*The Memory can become longer, in which case it becomes necessary to implement such calculation.*

The BIU obtains the logical address of a memory location from different sources depending on the type of reference that is being made.
Instructions always are fetched from the current code segment; `IP` then contains the offset of the target instruction from the beginning of the segment.
Stack instructions always operate on the current stack segment; `SP` contains the offset of the top of the stack.
Most variables are assumed to reside in the current data segment, although the program can instruct the BIU to access a variable in one of the other currently addressable segments.
The resultant address is called the effective address.

Strings are addressed differently than other variables.
The source operand of a string instruction is assumed to lie in the current data segment, but another currently addressable segment may be specified.
Its offset is taken from register `SI`, the source index register.
The destination operand of a string instruction always resides in the current data segment; its offset is taken from `DI`, the destination index register.
The string instructions automatically adjust `SI` and `DI` as they process the strings one byte or word at a time.

| Type of Memory Reference | Default Segment Base | Alternate Segment Base |      Offset       |
|:-------------------------|:--------------------:|:----------------------:|:-----------------:|
| Instruction Fetch        |         `CS`         |          None          |       `IP`        |
| Stack Operation          |         `SS`         |          None          |       `SP`        |
| Variable                 |         `DS`         |    `CS`, `ES`, `SS`    | Effective Address |
| String Source            |         `DS`         |    `CS`, `ES`, `SS`    |       `SI`        |
| String Destination       |         `ES`         |          None          |       `DI`        |
| BP Used as Base Register |         `SS`         |    `CS`, `DS`, `ES`    | Effective Address |

### Dynamically Relocated Code

The segment memory structure of the 8086 makes it possible to write programs that are position-independent, or dynamically relocated.
Dynamic relocations allows a multiprogramming or multitasking system to make particularly effective use of available resources.
Inactive programs can be written to disk and the space they occupied allocated to other programs/tasks.
If a disk-resident program is needed later, it can be read back into any available memory location and resumed.
Similarly, if a program needs a large contiguous block of memory, and the total amount is available in nonadjacent fragments, other program segments can be compacted to free up a continuous space.

In order to be dynamically relocated, a program must not load or alter its segment registers and must not transfer directly to a location outside the current code segment.
In other words, all offsets in the program must be relative to fixed values contained in the segment register.
This allows the program to be moved anywhere in memory as long as the segment registers are updated to point to the new base address.

### Stack Implementation

Stacks in the 8086 are implemented in memory and are located by the stack segment register `SS` and the stack pointer register `SP`.
a system may have an unlimited number of stacks, and a stack may be up to 64kB long, the maximum length of a segment.
Only one stack is addressable at a time; this is the current stack, often referred to simply as the stack.
`SS` contains the base address of the current stack and `SP` points to the top of the stack (TOS).
In other words, `SP` contains the offset of the top of the stack from the stack segment's base address.
Note, however, that the stack's base address contained in `SS` is not the "bottom" of the stack.

8086 stacks are 16-bit wide; instructions that operate on a stack add and remove stack items one word at a time.
An item is pushed onto the stack by **decrementing** `SP` by 2 and writing the item at the new TOS.
An item is popped off the stack by copying it from the TOS and then **incrementing** `SP` by 2.
In other words, the stack grows **down** in memory toward its base address.
Stack operations never move items on the stack, nor do they erase them. The top of the stack changes only as a result of updating the stack pointer.
