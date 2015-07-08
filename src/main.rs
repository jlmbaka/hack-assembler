use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;
use std::collections::HashMap;
use std::fmt;
use CommandType::{ACommand, CCommand, LCommand};


/// Main module that puts everything together and drives the entire translation process.
///
/// Made up of the following components:
///		* Parser
///		* Code module
/// 	* Symbol table
struct Assembler {
	parser: Parser,
	code: Code,
}

impl Assembler {
	fn new(filename: &str) -> Assembler {
		Assembler {
			parser: Parser::new(filename),
			code: Code::new(),
		}
	}

	/// Puts everything in motion.
	/// Contains the main program logc
	///
	/// TODO Consider moving it to a separte stand alone module.
	///
	///		Are there any more commands in the input?
	///			Reads the next command from the input and makes it the current command
	fn parse(&mut self) {
		loop {
			match self.parser.input_lines.next() { 
				Some(line) => { // has_more_command.true
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
						self.parser.current_command = content_without_inline;
					} else {
						self.parser.current_command = content;
					}

					match self.parser.command_type() {
						ACommand => {
							println!("ACommand: {0}", self.parser.current_command);
							println!("\tsymbol: {0}", self.parser.symbol());
						},
						CCommand => {
							println!("CCommand: {0}", self.parser.current_command);
							println!("\tdest: {0}", self.parser.dest());
							println!("\tcomp: {0}", self.parser.comp());
							println!("\tjump: {0}", self.parser.jump());
						},
						LCommand => {
							println!("LCommand: {0}", self.parser.current_command);
							println!("\tsymbol: {0}", self.parser.symbol());
						},
					}
				},
				None => break, // has_more_command.false
			}
		}
	}
}

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

	/// Returns the comp mnemonic in the current CCommand. dest=comp;jump
	///
	/// 28 possibilities.
	/// Should only be called when cammand_type() is CCommand.
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
struct Code {
	c_instr: CInstruction,
}

impl Code {
	fn new() -> Code {
		Code {
			c_instr: CInstruction::new(),
		}
	}

	/// Reset all the bits to 0
	fn clear(mut self) {
		self.c_instr = CInstruction::new();
	}

	/// Gives the binary representation of the current instruction as string.
	///
	/// 111ac1c2c3c4c5c6d1d2d3j1j2j3
	fn format(self) {
		println!("{0}", self.c_instr);
	}

	/// Returns the binary code of the dest mnemonic
	///
	/// returns 3 bits
	fn dest(mut self, mnemonic: &str) -> CInstruction {
		match mnemonic {
			"null" 	=> {},
			"M"		=> {
				self.c_instr.j3 = 1;
			},
			"D"		=> {
				self.c_instr.j2 = 1;
			},
			"MD"	=> {
				self.c_instr.j3 = 1;
				self.c_instr.j2 = 1;
			},
			"A"		=> {
				self.c_instr.j1 = 1;
			},
			"AM"	=> { 
				self.c_instr.j1 = 1;
				self.c_instr.j3 = 1;
			},
			"AD"	=> {
				self.c_instr.j1 = 1;
				self.c_instr.j2 = 1;
			},
			"AMD"	=> {
				self.c_instr.j1 = 1;
				self.c_instr.j2 = 1;
				self.c_instr.j3 = 1;
			},
			_		=> {},
		}
		self.c_instr
	}

	/// Returns the binary code of the comp mnemonic
	///
	/// returns 7 bits
	fn comp(mut self, mnemonic: &str) -> CInstruction {
		match mnemonic {
			"0" => {
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"1" => {
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 1;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 1;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 1;
			},
			"-1" => {
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 1;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"D" => {
				self.c_instr.c1 = 0;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 1;
				self.c_instr.c5 = 0;
				self.c_instr.c6 = 0;
			},
			"A" | "!M" => {
				if mnemonic == "!M" {self.c_instr.a = 1;}
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"!D" => {
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"!A" => {
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"-D" => {
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"-A" | "-M" => {
				if mnemonic == "-M" {self.c_instr.a = 1;}
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"D+1" => {
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"A+1" | "M+1" => {
				if mnemonic == "M+1" {self.c_instr.a = 1;}
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"D-1" => {
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"A-1" | "M-1" => {
				if mnemonic == "M-1" {self.c_instr.a = 1;}
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"D+A" | "D+M" => {
				if mnemonic == "D+M" {self.c_instr.a = 1;}
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"D-A" | "D-M" => {
				if mnemonic == "D-M" {self.c_instr.a = 1;}
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"A-D" | "M-D" => {
				if mnemonic == "M-D" {self.c_instr.a = 1;}
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"D&A" | "D&M" => {
				if mnemonic == "D&M" {self.c_instr.a = 1;}
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			"D|A" | "D|M" => {
				if mnemonic == "D|M" {self.c_instr.a = 1;}
				self.c_instr.c1 = 1;
				self.c_instr.c2 = 0;
				self.c_instr.c3 = 1;
				self.c_instr.c4 = 0;
				self.c_instr.c5 = 1;
				self.c_instr.c6 = 0;
			},
			_ => {},
		}
		self.c_instr
	}

	/// Returns the binary code of the jump mnemonic
	///
	/// returns 3 bits
	fn jump(mut self, mnemonic: &str) -> CInstruction {
		match mnemonic {
			"null" 	=> {},
			"JGT"	=> {
				self.c_instr.j3 = 1;
			},
			"JEQ"	=> {
				self.c_instr.j2 = 1;
			},
			"JGE"	=> {
				self.c_instr.j3 = 1;
				self.c_instr.j2 = 1;
			},
			"JLT"	=> {
				self.c_instr.j1 = 1;
			},
			"JNE"	=> {
				self.c_instr.j1 = 1;
				self.c_instr.j3 = 1;
			},
			"JLE"	=> {
				self.c_instr.j1 = 1;
				self.c_instr.j2 = 1;
			},
			"JMP"	=> {
				self.c_instr.j1 = 1;
				self.c_instr.j2 = 1;
				self.c_instr.j3 = 1;
			},
			_		=> {},
		}
		self.c_instr
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

impl fmt::Display for CInstruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "111{0}{1}{2}{3}{4}{5}{6}{7}{8}{9}{10}{11}{12}",
			self.a, 
			self.c1, self.c2, self.c3,self.c4,self.c5, self.c6,
			self.d1, self.d2, self.d3,
			self.j1, self.j2, self.j3)
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("HACK Assembler. Translates assembly (mnemonic) into binary 
			machine code.\n\nUsage:\n\tassembler [PATH_TO_ASM_FILE]");
		return;
	}
	let path_to_asm_file = &args[1];
	let mut assembler = Assembler::new(path_to_asm_file);
	assembler.parse();
}
