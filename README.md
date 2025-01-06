# `sim8086`
**sim8086** is a lightweight simulator for the 8086 family of Intel processors, built to decode raw machine code into 8086 Assembly and emulate execution as it would occur on real hardware.

<p align="center">
  <img src="assets/gradient.jpg" alt="Gradient Screenshot" title="Sim8086 gradient rendering" />
</p>

This project is primarily educational, as it is important to have a model of how the CPU works and the language it uses. 
This is primarily helpful when one wants to write efficient software, work with embedded systems or implement a low level process, like an asynchronous runtime. 
sim8086 provides an introduction to these concepts, and the accompanying
[instruction manual](https://edge.edx.org/c4x/BITSPilani/EEE231/asset/8086_family_Users_Manual_1_.pdf)
serves as a handy reference.

Although sim8086 does not yet implement every single 8086 instruction, it covers most of them well enough to handle typical programs of moderate complexity. 
I encourage you to consult the instruction manual and consider contributing to improve and extend the simulator.
For all instruction for which decoding and execution has been implemented, there are accompanying tests in `sim8086/tests/`.
To run the decoding tests you must have [NASM](https://www.nasm.us/) installed. 
Additionally, there is a test file that contains all available instructions `sim8086/tests/decode/test_all`, and currently fails, 
because of runtime errors which will signify which instructions' decoding has not been implemented yet.

I might continue extending the simulator myself from time to time.

## Installation

To install sim8086 clone this repository and build it with `cargo` either in `debug` or `release` mode.
```
git clone https://github.com/gsmyridis/sim8086.git
cd sim8086
cargo build
```
The binary is then located in `target/debug/sim8086` or `target/release/sim8086` depending on the build flag you used.

## Usage

sim8086 has two main commands: `decode` and `execute`.

### Decoding

The `decode` command reads a byte-code file (raw 8086 machine code) and produces an Assembly listing. 
The byte-code is decoded into intermediate representations which then are displayed as regular 8086 Assembly.
```
sim8086 decode <INPUT> --output <OUTPUT>
```
- `<INPUT>`: The path to the binary file containing the 8086 machine code.
- `<OUTPUT>`: Path to the file where the resulting Assembly code will be written. If no output file is provided, the listing will be displayed in `STDOUT`.

### Executing

The `execute` command runs the 8086 machine code under the simulator, allowing you to see the program’s behavior.
In `STDOUT` you will see the 8086 Assembly code along with the final state of the CPU.
```
sim8086 execute <INPUT> --output <OUTPUT>
```
- `<INPUT>`: The path to the binary file containing the 8086 machine code to simulate.
- `<OUTPUT>`: The path to the file to dump the memory content. If no output is provided then you can only inspect the final state of the CPU in `STDOUT`.

`sim8086` cannot execute Assembly listings, it can only decode byte-code into intermediate representations which are then executed.

## Image Renderer! 

Although `sim8086` doesn’t decode and execute every instruction yet, it can already handle reasonably complex programs.
For example, the test program `sim8086/tests/data/execute/draw_rectangle` generates a simple 64×64 gradient image. 
To see it in action, run the binary with `sim8086` and dump the memory into a `.data` file. 
Next, open that file in GIMP as an “RGB Alpha” image. 
Skip the first 256 bytes (which contain the rendering code), and set the image dimensions to 64×64. 
Voila! You’ll see the rendered gradient at the top of the page.

<p align="center">
  <img src="assets/gradient.jpg" alt="Gradient Screenshot" title="Sim8086 gradient rendering" />
</p>
