# `sim8086`
**sim8086** is a lightweight simulator for the 8086 family of Intel processors, built to decode raw machine code into 8086 Assembly and emulate execution as it would occur on real hardware.

This project is primarily educational: to write efficient software, it helps to understand how the CPU operates at a low level. 
By reading and analyzing assembly, you can spot any inefficiencies or “waste” in your program. 
sim8086 provides an introduction to these concepts, and the accompanying instruction manual serves as a handy reference.

Although sim8086 does not yet implement every single 8086 instruction, it covers most of them well enough to handle typical programs of moderate complexity. 
I encourage you to consult the instruction manual and consider contributing to improve and extend the simulator.
For all instruction for which decoding and execution has been implemented, there are accompanying tests in `sim8086/tests/`.

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
