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
						ACommand => {
							println!("ACommand: {0}", self.current_command);
							println!("\tsymbol: {0}", self.symbol());
						},
						CCommand => {
							println!("CCommand: {0}", self.current_command);
							println!("\tdest: {0}", self.dest());
							println!("\tcomp: {0}", self.comp());
							println!("\tjump: {0}", self.jump());

						},
						LCommand => {
							println!("LCommand: {0}", self.current_command);
							println!("\tsymbol: {0}", self.symbol());

						},
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
	/// * LCommand: Pseudo-Command. For (xxx) where xxx is a symbol
	fn command_type(&self) -> CommandType {
		if self.current_command.starts_with("@") {
			return ACommand
		} else if self.current_command.starts_with("(") {
			return LCommand
		}
		CCommand
	}

	/// Returns the symbol or decimal xxx of the current command @xxx or (xxx).
	///
	/// Should be called only when command_type() is ACommand or LCommand.
	fn symbol(&self) -> String {
		let pattern: &[_] = &['(', ')', '@'];
		self.current_command.trim_matches(pattern).to_string()
	}

	/// Returns the dest mnemonic in the current CCommand. dest=comp;jump
	///  
	///(8 posibilities.)
	/// Should only be called when cammand_type() is CCommand.
	fn dest(&self) -> String {
		// dest or jump field may be empy
		// if dest is empty, the '=' is omitted.
		// if jump is empty, the ';' is omitted.
		match self.current_command.contains('=') {
			true => {
				let v: Vec<&str> = self.current_command.split('=').collect();
				return v[0].to_string()
			},
			false => "null".to_string(),
		}
	}

	// /// Returns the comp mnemonic in the current CCommand. dest=comp;jump
	// ///
	// /// 28 possibilities.
	// /// Should only be called when cammand_type() is CCommand.
	fn comp(&self) -> String {
		match self.current_command.contains('=') {
			true => {
				match self.current_command.contains(';') {
					true => { // dest=comp;jump	
						let v: Vec<&str> = self.current_command.split(|c: char| c == ';' || c == '=' ).collect();
						v[1].to_string()					
					},
					false => { // dest=comp
						let v: Vec<&str> = self.current_command.split('=').collect();
						v[1].to_string()
					}
				}
			},
			false => {
				match self.current_command.contains(';') {
					true => { // comp;jump
						let v: Vec<&str> = self.current_command.split(';').collect();
						v[0].to_string()
					},
					false => { // comp
						self.current_command.clone()
					},
				}
			}
		}
	}

	/// Returns the jump mnemonic in the current CCommand. dest=com;jump
	///
	/// 8-possibilities.
	/// Should only be called when cammand_type() is CCommand.
	fn jump(&self) -> String {
		match self.current_command.contains(';') {
			true => {
				let v: Vec<&str> = self.current_command.split(';').collect();
				return v[0].to_string()
			},
			false => "null".to_string(),
		}	
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
