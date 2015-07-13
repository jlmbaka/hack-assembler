extern crate hack;
use hack::assembler::Assembler;

use std::path::Path;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;

// /// FEATURE
// /// As a user,
// /// I want to be able to translate assembly that does not contain symbols to binary code.
// #[test]
// fn translate_asm_file_to_hack_no_symbols() {
// 	let filenames_no_symbol: Vec<&str> = vec!["Add", "MaxL", "RectL", "PongL"];
// 	generic_asm_to_hack(filenames_no_symbol);
// }

/// FEATURE
/// As a user,
/// I want to be able to translate assembly that contains symbols to binary code.
#[test]
fn translate_asm_file_to_hack_symbols() {
	let filenames_with_symbol: Vec<&str> = vec!["Rect", "Max", "Pong"];
	generic_asm_to_hack(filenames_with_symbol);
}

fn generic_asm_to_hack(filenames_no_symbol: Vec<&str>) {
	let dir = "06/";
	let in_ext = ".asm";
	let out_ext = ".hack";
	let expected_dir = "expected/";

	// GIVEN I am a user
	// let filenames_no_symbol: Vec<&str> = vec!["Add", "MaxL", "RectL", "PongL"];
	for filename in &filenames_no_symbol {
		// WHEN I run the assembler with a file argument "{0}.asm"
		let f_in = dir.to_string() + filename + in_ext;
		let mut assembler = Assembler::new(&f_in);
		assembler.translate();

		// THEN I will get an output file named "{0}.hack" containing the correct binary code of the given input file.
		let f_out = dir.to_string() + filename + out_ext;
		let f_expected = dir.to_string() + expected_dir + filename + out_ext;

		let actual = load_file_content(&f_out);
		let expected = load_file_content(&f_expected);

		assert_eq!(actual, expected);

		// clean_up(&f_out);
	}
}

fn load_file_content(filename: &str) -> String {
	let path = Path::new(filename);
	let display = path.display();
	let mut file = match OpenOptions::new().read(true).open(filename) {
		Ok(file) => file,
		Err(why) => panic!("couldn't open the file {}: {}", display, Error::description(&why)),
	};

	let mut s = String::new();
	match file.read_to_string(&mut s) {
		Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
		Ok(_) => print!("{} contains:\n{}", display, s),
	};
	s = s.replace("\r", "");
	s
}

fn clean_up(filename: &str) {
	match std::fs::remove_file(filename) {
		Ok(_) => {"Clean up successful";},
		Err(_) => {"Couldn't clean up";},
 	}
}