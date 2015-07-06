use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;
use std::collections::HashMap;

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

	/// Contains the main program logc
	///
	/// TODO Consider moving it to a separte stand alone module.
	///
	///		Are there any more commands in the input?
	///			Reads the next command from the input and makes it the current command
	fn parse(&mut self) {
		loop {
			match self.input_lines.next() { 
				Some(line) => {
					let content = line.unwrap().trim().to_string();

					// ignore line comments and empty lines
					if content.starts_with("//") || content == "\n" || content == "" {
						continue
					}


					// remove inline commnents
					let mut content_without_inline = String::new();
					if content.contains("//") {
						let v: Vec<&str> = content.split("//").collect();
						content_without_inline = v[0].trim().to_string();
					}

					// decide whether to use content_with_inline or content
					if content_without_inline != String::new() {
						self.current_command = content_without_inline;
					} else {
						self.current_command = content;
					}

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
				return v[1].to_string()
			},
			false => "null".to_string(),
		}	
	}
}

/// Translate Hack assembly language mnemonic into binary codes
struct Code;

impl Code {
	fn new() -> Code {
		Code
	}

	/// Returns the binary code of the dest mnemonic
	///
	/// returns 3 bits
	fn dest(mnemonic: &str) -> u8 {
		match mnemonic {
			"null" 	=> 0x00,
			"M"		=> 0x01,
			"D"		=> 0x02,
			"MD"	=> 0x03,
			"A"		=> 0x04,
			"AM"	=> 0x05,
			"AD"	=> 0x06,
			"AMD"	=> 0x07,
			_		=> 0x08,
		}
	}

	/// Returns the binary code of the comp mnemonic
	///
	/// returns 7 bits
	// fn comp(mnemonic: &str) -> u8 {

	// }

	/// Returns the binary code of the jump mnemonic
	///
	/// returns 3 bits
	fn jump(mnemonic: &str) -> CInstruction {
		let mut c_instr = CInstruction::new();
		match mnemonic {
			"null" 	=> ,
			"JGT"	=> {
				c_instr.j3 = 1;
			},
			"JEQ"	=> {
				c_instr.j2 = 1;
			},
			"JGE"	=> {
				c_instr.j3 = 1;
				c_instr.j2 = 1;
			},
			"JLT"	=> {
				c_instr.j1 = 1;
			},
			"JNE"	=> {
				c_instr.j1 = 1;
				c_instr.j3 = 1;
			},
			"JLE"	=> {
				c_instr.j1 = 1;
				c_instr.j2 = 1;
			},
			"JMP"	=> {
				c_instr.j1 = 1;
				c_instr.j2 = 1;
				c_instr.j3 = 1;
			},
			_		=> ,
		}
		c_instr
	}
}

/// Change the bits at indexes in index to values in dest
///
/// Takes a hashmap such as bit_index -> bit_value
fn set_bits(mut word:i16, index_bitvalue: HashMap<i16, i16>) -> i16 {
	for index in index_bitvalue.keys() {
		let bit_value: i16 = match index_bitvalue.get(index) {
			Some(value) => value.clone(),
			None => 0,
		};
		word ^= (-bit_value ^ word) & (1i16 << index);
	}
	word
}

/// CInstruction bitfield
struct CInstruction {
	a: u8,
	c1: u8,
	c2: u8,
	c3: u8,
	c4: u8,
	c5: u8,
	c6: u8,
	d1: u8,
	d2: u8,
	d3: u8,
	j1: u8,
	j2: u8,
	j3: u8,
}

impl CInstruction {
	fn new() -> CInstruction {
		CInstruction {
			a: 0,
			c1: 0,
			c2: 0,
			c3: 0,
			c4: 0,
			c5: 0,
			c6: 0,
			d1: 0,
			d2: 0,
			d3: 0,
			j1: 0,
			j2: 0,
			j3: 0,
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("HACK Assembler. Translates assembly (mnemonic) into binary 
			machine code.\n\nUsage:\n\tassembler [PATH_TO_ASM_FILE]");
		return;
	}
	let path_to_asm = &args[1];
	let mut parser = Parser::new(path_to_asm);
	parser.parse();
}
