# hack-assembler

A simple assembler for the Hack Platform, written in Rust.

I have implemented this assembler based on the book *The Elements of Computing Systems* by Noam Nisam and Shimon Schocken.

It translates Hack Assembly (mnemonics) into Binary machine code for the Hack computer.

## What is it?

The Hack Assembler provides the facility to translate Hack Assembly to Binary representation. This binary representation is understood by the Hack Hardware platform.

The Hack computer is a Von Neumann platform. It is a 16-bit machine, consisting of a CPU, two separate memory modules serving as instruction memory and data memory, and two memory-mapped I/O devices (screen and keyboard).


## Documentation

The Hack computer platform is fully documented in the book *The Elements of Computing Systems* by Noam Nisam and Shimon Schocken and its companion [website](http://www.nand2tetris.org) that contains resources for the associated course, projects and supporting software. The goal of the authors is to guide the reader through the construction of a computer from the ground up, going from Boolean principles to a fully functional Tetris game.

## Installation

Since the program is provided as source, you will need to compile it to your target platform. Therefore, you need to have a fully working installation of [Rust](http://www.rust-lang.org). Once it is compiled, you can run it from the command prompt without the need to install it. See the next section for usage instruction.

## Usage

The Hack assembler reads as input a text file named `<name_of_prog>.asm` containing a Hack assembly program, and outputs a text file named `<name_of_prog>.hack`, containing the translated Hack machine code.

The input file is supplied to the assembler program as a command line argument

```
prompt> assembler [PATH_TO_ASM_FILE]

```

## Licensing


Please see the file called LICENSE.



## Contacts

