# `sim8086` 

#

The 8086 are third-generation processors. 
Mircroprocessors generally execute a program by repeatedly cycling through the simplified steps below:

1. Fetch the next instruction from memory.
2. Read an operand (if required by the instruction).
3. Execute the instruction.
4. Write the result (if required by the instruction).

In CPUs prior to 8086, most of these steps have been performed serially, or with only a single bus cycle fetch overlap.
The architecture of the 8086, while performing the same steps, allocates them to two separate processing units within the CPU.
The execution unit (EU) executes instructions, and the bus interface unit (BUI) fetches instructions, reads operands and writes results. 

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
In fact, all eight general registers fit the definition of "accumulator" as used in first and s