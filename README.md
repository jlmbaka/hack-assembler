# hack-assembler
<<<<<<< HEAD
Hack Assembler. Translates Hack Assembly (mnemonics) into Binary machine code for the Hack Platform.

## What is it?

The Hack Assembler provides the facility to translate Hack Assembly to Binary representation. This binary representation is understood by the Hack Hardware platform.

The Hack computer is a Von Neumann platform. It is a 16-bit machine, consisting of a CPU, two separate memory modules serving as instruction memory and data memory, and two memory-mapped I/O devices (screen and keyboard).


## Documentation

The Hack computer platform is fully documented in the book *The Elements of Computing Systems* by Noam Nisam and Shimon Schocken and its companion [website](http://www.nand2tetris.org) that contains ressources for the associated course, projects and supporting software. The goal of the authors is to guide the reader through the construction of a computer from the ground up, going from boolean principles to a fully functional Tetris game.

## Installation

Since the program is provided as source, you will need to compile it to your target platform. Therefore, you need to have a fully working installation of [Rust](http://www.rust-lang.org). Once it is compiled, you can run it from the command prompt without the need to install it. See the next section for usage instruction.

## Usage

The Hack assembler reads as input a text file named `<name_of_prog>.asm` containing a Hack assembly program, and outpus a text file named `<name_of_prog>.hack`, containing the translated Hack machine code.

The input file is supplied to the assembler program as a command line argument

```
prompt> assember [PATH_TO_ASM_FILE]

```

## Licensing


Please see the file called LICENSE.



## Contacts
=======
Hack Assembler. Translates Hack assembly (mnemonics) into binary machine code for the Hack Platform.

## Usage


```
assembler [PATH_TO_ASM_FILE]
```

## Contributing


1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request :D

## History


## Credits


## Licence
>>>>>>> symbol-table

