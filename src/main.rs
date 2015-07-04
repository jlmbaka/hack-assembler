use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;

use CommandType::{A_COMMAND, C_COMMAND, L_COMMAND};

enum CommandType {
	A_COMMAND,
	C_COMMAND,
	L_COMMAND,
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
		Parser { input_lines: lines, current_command: String::new()}
	}

	/// Are there any more commands in the input?
	///
	/// NOTE: Not needed since input_lines is an iterator
	fn has_more_commands(&self) -> bool {
		false
	}

	/// Reads the next command from the input and makes it the current command.
	///
	/// Should be called only if hasMoreCommands() is true.
	/// Initally there is no current command
	///
	fn advance(&mut self) {
		// Lines.next() -> Option<Result<String>>. That is why we have two unwrap()
		let s = self.input_lines.next().unwrap().unwrap();
		let s_trimmed = s.trim();
		if s_trimmed.starts_with("//") {
			println!("{0}", s_trimmed);			
		} else {
			println!("No comment");
		}
	}


	/// Encapsulates has_more_command and avance method from the proposed API.
	///
	fn parse(mut self) {
		for line in self.input_lines { // has_more_commands + advance
			// println!("{0}", line.unwrap().trim());
			self.current_command = line.unwrap().trim().to_string();
			match self.command_type() {
				A_COMMAND 	=> {println!("A_COMMAND: {0}", self.current_command);},
				C_COMMAND 	=> {println!("C_COMMAND: {0}", self.current_command);},
				L_COMMAND 	=> {println!("L_COMMAND: {0}", self.current_command);},
			}
		}
	}

	/// Returns the type of the current command.
	///
	/// * A_COMMAND: For @xxx where xxx is either a symbol or a decimal number
	/// * C_COMMAND: For dest=comp;jump
	/// * L_COMMAND: Pseudo-Command. for (xxx) where xxx is a symbol
	fn command_type(&self) -> CommandType {
		A_COMMAND
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Usage: assembler [PATH_TO_ASM_FILE]");
		return;
	}
	let path_to_asm = &args[1];
	let mut parser = Parser::new(path_to_asm);
	parser.parse();
}
