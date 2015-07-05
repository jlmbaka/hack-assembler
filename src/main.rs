use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;

use CommandType::{ACommand, CCommand, LCommand};

enum CommandType {
	ACommand,
	CCommand,
	LCommand,
}

struct Parser {
	input_lines: Lines<BufReader<File>>,
	current_command: String,
}

impl Parser {
	/// Opens the input file/stream and gets ready to parse it.
	fn new(filename: &str) -> Parser {
		let path = Path::new(filename);
		let file = match File::open(&path) {
			Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
			Ok(file) => file,
		};

		let lines = BufReader::new(file).lines(); // iterator
		Parser {
			input_lines: lines, 
			current_command: String::new(),
		}
	}

	/// Encapsulates has_more_command and avance method from the proposed API.
	///
	///		Are there any more commands in the input?
	///			Reads the next command from the input and makes it the current command
	fn parse(&mut self) {
		loop {
			match self.input_lines.next() {
				Some(line) => {
					let content = line.unwrap().trim().to_string();

					// Ignore comments and empty lines
					if content.starts_with("//") || content == "\n" || content == "" {
						continue
					}

					self.current_command = content;
					match self.command_type() {
						ACommand => {println!("ACommand: {0}", self.current_command);},
						CCommand => {println!("CCommand: {0}", self.current_command);},
						LCommand => {println!("LCommand: {0}", self.current_command);},
					}
				},
				None => break,
			}
		}
	}

	/// Returns the type of the current command.
	///
	/// * ACommand: For @xxx where xxx is either a symbol or a decimal number
	/// * CCommand: For dest=comp;jump
	/// * LCommand: Pseudo-Command. for (xxx) where xxx is a symbol
	fn command_type(&self) -> CommandType {
		if self.current_command.starts_with("@") {
			return ACommand
		} else if self.current_command.starts_with("(") {
			return LCommand
		}
		CCommand
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("HACK Assembler. Translates assembly (mnemonic) into binary machine code.\n\nUsage:\n\tassembler [PATH_TO_ASM_FILE]");
		return;
	}
	let path_to_asm = &args[1];
	let mut parser = Parser::new(path_to_asm);
	parser.parse();
}
