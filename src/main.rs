extern crate hack;

use std::env;
use hack::assembler::Assembler;

#[cfg(not(test))]
fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("HACK Assembler. Translates assembly (mnemonics) into binary machine code.\n
			\nUsage:\n\tassembler [PATH_TO_ASM_FILE]");
		return;
	}
	let path_to_asm_file = &args[1];
	let mut assembler = Assembler::new(path_to_asm_file);
	assembler.translate();
}