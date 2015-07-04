use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::env;


struct Parser {
	input_file: File,
}

impl Parser {
	fn new(filename: &str) -> Parser {
		let path = Path::new(filename);
		let file = match File::open(&path) {
			Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
			Ok(file) => file,
		};

		Parser { input_file: file}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Usage: assembler [PATH_TO_ASM_FILE]");
		return;
	}
	let path_to_asm = &args[1];
	let parser = Parser::new(path_to_asm);
}
