# `sim8086` 

## Instructions

To pack instructions into memory aas densely as possible, the 8086 and 8088 CPU utilize an efficient coding technique.
Machine instructions vary from one to six bytes in length.
One-byte instructions, which generally operate on single registers or flags, are simple to identify.
The keys decoding longer instructions are in the first two bytes. 
The format of these bytes can vary, but most instructions follow the format shown below.
The first six bits of a multibyte instruction generally contains an opcode that identifies the basic instruction type: `ADD`, `XOR`, etc.
The following bit, called the `D` field, generally specifies "direction" of the operation.
When `D=1` the `REG` field in the second byte identifies the destination operand, while when `D=0` the `REG` field identifies the source operand.
The next bit, `W` for "wide", distinguishes between byte and word operation.
When `W=1` the operation is applied on a word, otherwise on a byte.

One of three additional single-bit fields, `S`, `V` or `Z`, appears in some instruction formats.
`S` is used in conjuction with `W` to indicate sign extension of immediate fields in arithmetic instructions.
`V` distinguishes between single- and variable-bit shifts and rotates.
`Z` is used as a compare bit the zero flag is conditional repeat and loop instructions.
All single-bit field settings are summarized in the following table:



### `mov`


Writing a `mov` instruction in ASM-86 in the form:

```
mov destination, source
```

will cause the assembler to generate 1 of 28 possible forms of the `mov` machine instruction.


