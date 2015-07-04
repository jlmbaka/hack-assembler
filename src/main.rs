use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;


struct Parser {
	input_lines: Lines<BufReader<File>>,
}

impl Parser {
	///
	/// Opens the input file/stream and gets ready to parse it.
	///
	fn new(filename: &str) -> Parser {
		let path = Path::new(filename);
		let file = match File::open(&path) {
			Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
			Ok(file) => file,
		};

		let lines = BufReader::new(file).lines(); // iterator
		Parser { input_lines: lines}
	}

	///
	/// Are there any more commands in the input?
	/// NOTE: Not needed since we input_lines is an iterator
	fn has_more_commands(&self) -> bool {
		false
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
