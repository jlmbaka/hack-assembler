use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Lines;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;

enum CommandType {
	ACommand,
	CCommand,
	LCommand,
}

/// Main module that puts everything together and drives the entire translation process.
///
/// Made up of the following components:
///		* Parser
///		* Code module
/// 	* Symbol table
///
/// Two-pass assembler that reads the code twice from start to end.
///
/// * First pass:
/// build the symbol table and generates no code.
////
/// * Second pass:
/// all the label symbols encountered in the program have already
/// been bound to memory locations and recorded in the symbol table. Thus the 
/// assembler can replace each symbol with its corresponding meaning (numeric addr)
/// and generate the final binary code.
pub struct Assembler {
	parser: Parser,
	input_filename: String,
	symbol_table: SymbolTable,
}

impl Assembler {
	pub fn new(filename: &str) -> Assembler {
		Assembler {
			parser: Parser::new(filename),
			input_filename: filename.to_string(),
			symbol_table: SymbolTable::new(),
		}
	}

	/// Put together all the steps of the translation process.
	///
	/// Inialise Symbol Table -> First Pass -> Second Pass
	pub fn translate(&mut self)  {
		self.symbol_table.initialise();
		println!("----------------- FIRST PASS ----------------");
		self.first_pass();
		println!("----------------- SECOND PASS ----------------");
		self.second_pass();
	}

	/// Open a file
	///
	/// returns the opened has been opened
	fn open_file(filename: &str) -> File{
		let path = Path::new(filename);
		let display = path.display();

		let file = match OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open(&path) {
			Err(why) => panic!("Error on file {}: {}", display, Error::description(&why)),
			Ok(file) => file,		
		};
		file
	}

	/// Write to file
	fn write_to_file(mut file: &File, code: u16) {		
		match file.write_fmt(format_args!("{:016b}\n", code)) {
			Err(why) => panic!("couldn't write to file: {}", Error::description(&why)),
			Ok(_) => (),
		}
	}

	fn generate_output_filename(&self) -> String {
		let v: Vec<&str> = self.input_filename.rsplitn(2, ".asm").collect();
		let output_ext = ".hack";
		let output_filename = v[1].to_string() + output_ext;
		output_filename
	}

	/// Go through the entire assembly program, line by line, and build the symbol table.
	///
	/// Doesn't generate any code.
	fn first_pass(&mut self) {
		let mut rom_address: isize = -1;
		loop {
			match self.parser.input_lines.next() {
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

					// decide whether to use content_without_inline or content
					if content_without_inline != String::new() {
						self.parser.current_command = content_without_inline;
					} else {
						self.parser.current_command = content;
					}

					match self.parser.command_type() {
						CommandType::ACommand | CommandType::CCommand => {
							rom_address += 1;
						},
						CommandType::LCommand => {
							let l_cmd_sym = self.parser.symbol();
							let next_instr_rom_address = (rom_address + 1) as u16;
							println!("({0}, {1})", l_cmd_sym.clone(), next_instr_rom_address.clone());
							self.symbol_table.add_entry(l_cmd_sym, next_instr_rom_address);
						},
					}
				},
				None => break,
			}
		}
	}

	/// Puts everything in motion.
	///
	/// Contains the main program logic.
	///		Are there any more commands in the input.
	///			Reads the next command from the input and makes it the current command.
	fn second_pass(&mut self) {
		// get a new parser
		self.parser = Parser::new(&self.input_filename);

		// file where the translated assembly will be written to. 
		let output_filename = self.generate_output_filename();

 		// closed when binding goes out of scope.
		let output_file = Assembler::open_file(&output_filename); 

		// user variable addr
		let mut user_defined_var_addr: u16 = 16;

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
						CommandType::ACommand => {
							println!("CommandType::ACommand: {0}", self.parser.current_command);

							let a_cmd_symbol = self.parser.symbol();

							match a_cmd_symbol.parse::<u16>() {
								Ok(address) => Assembler::write_to_file(&output_file, address),
								Err(_) => { // symbol aka @symbol
									match self.symbol_table.contains(&a_cmd_symbol) {
										true => { // found the symbol in the table
											let address = self.symbol_table.get_address(&a_cmd_symbol);
											Assembler::write_to_file(&output_file, address);
										},
										false => { // couldn't find the symbol in the table. Find the next available addr.
											self.symbol_table.add_entry(a_cmd_symbol, user_defined_var_addr);
											Assembler::write_to_file(&output_file, user_defined_var_addr);
											user_defined_var_addr += 1;
										},
									}
								}
							}
						},
						CommandType::CCommand => {
							println!("CommandType::CCommand: {0}", self.parser.current_command);

							let dest = Code::dest(&self.parser.dest());
							let comp = Code::comp(&self.parser.comp());
							let jump = Code::jump(&self.parser.jump());

							let c_instr = "111".to_string() + &(comp.to_string()) + &(dest.to_string()) + &(jump.to_string());

							Assembler::write_to_file(&output_file, u16::from_str_radix(&c_instr, 2).unwrap());
						},
						CommandType::LCommand => {},
					}
				},
				None => break,
			}
		}
	}
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
			return CommandType::ACommand
		} else if self.current_command.starts_with("(") {
			return CommandType::LCommand
		}
		CommandType::CCommand
	}

	/// Returns the symbol or decimal xxx of the current command @xxx or (xxx).
	///
	/// Should be called only when command_type() is CommandType::ACommand or CommandType::LCommand.
	fn symbol(&self) -> String {
		let pattern: &[_] = &['(', ')', '@'];
		self.current_command.trim_matches(pattern).to_string()
	}

	/// Returns the dest mnemonic in the current CommandType::CCommand. dest=comp;jump
	///  
	/// 8 posibilities.
	/// Should only be called when cammand_type() is CommandType::CCommand.
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

	/// Returns the comp mnemonic in the current CommandType::CCommand. dest=comp;jump
	///
	/// 28 possibilities.
	/// Should only be called when cammand_type() is CommandType::CCommand.
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

	/// Returns the jump mnemonic in the current CommandType::CCommand. dest=com;jump
	///
	/// 8-possibilities.
	/// Should only be called when cammand_type() is CommandType::CCommand.
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

	/// Returns the binary code of the dest mnemonic
	///
	/// returns 3 bits
	fn dest(mnemonic: &str) -> Dest {
		let mut dest_bits = Dest::new();
		match mnemonic {
			"null" 	=> {},
			"M"		=> {
				dest_bits.d3 = 1;
			},
			"D"		=> {
				dest_bits.d2 = 1;
			},
			"MD"	=> {
				dest_bits.d3 = 1;
				dest_bits.d2 = 1;
			},
			"A"		=> {
				dest_bits.d1 = 1;
			},
			"AM"	=> { 
				dest_bits.d1 = 1;
				dest_bits.d3 = 1;
			},
			"AD"	=> {
				dest_bits.d1 = 1;
				dest_bits.d2 = 1;
			},
			"AMD"	=> {
				dest_bits.d1 = 1;
				dest_bits.d2 = 1;
				dest_bits.d3 = 1;
			},
			_		=> {},
		}
		dest_bits
	}

	/// Returns the binary code of the comp mnemonic
	///
	/// returns 7 bits
	fn comp(mnemonic: &str) -> Comp {
		let mut comp_bits = Comp::new();
		match mnemonic {
			"0" => {
				comp_bits.c1 = 1;
				comp_bits.c2 = 0;
				comp_bits.c3 = 1;
				comp_bits.c4 = 0;
				comp_bits.c5 = 1;
				comp_bits.c6 = 0;
			},
			"1" => {
				comp_bits.c1 = 1;
				comp_bits.c2 = 1;
				comp_bits.c3 = 1;
				comp_bits.c4 = 1;
				comp_bits.c5 = 1;
				comp_bits.c6 = 1;
			},
			"-1" => {
				comp_bits.c1 = 1;
				comp_bits.c2 = 1;
				comp_bits.c3 = 1;
				comp_bits.c4 = 0;
				comp_bits.c5 = 1;
				comp_bits.c6 = 0;
			},
			"D" => {
				comp_bits.c1 = 0;
				comp_bits.c2 = 0;
				comp_bits.c3 = 1;
				comp_bits.c4 = 1;
				comp_bits.c5 = 0;
				comp_bits.c6 = 0;
			},
			"A" | "M" => {
				if mnemonic == "M" {comp_bits.a = 1;}
				comp_bits.c1 = 1;
				comp_bits.c2 = 1;
				comp_bits.c3 = 0;
				comp_bits.c4 = 0;
				comp_bits.c5 = 0;
				comp_bits.c6 = 0;
			},
			"!D" => {
				comp_bits.c1 = 0;
				comp_bits.c2 = 0;
				comp_bits.c3 = 1;
				comp_bits.c4 = 1;
				comp_bits.c5 = 0;
				comp_bits.c6 = 1;
			},
			"!A" | "!M" => {
				if mnemonic == "!M" {comp_bits.a = 1;}
				comp_bits.c1 = 1;
				comp_bits.c2 = 1;
				comp_bits.c3 = 0;
				comp_bits.c4 = 0;
				comp_bits.c5 = 0;
				comp_bits.c6 = 1;
			},
			"-D" => {
				comp_bits.c1 = 0;
				comp_bits.c2 = 0;
				comp_bits.c3 = 1;
				comp_bits.c4 = 1;
				comp_bits.c5 = 1;
				comp_bits.c6 = 1;
			},
			"-A" | "-M" => {
				if mnemonic == "-M" {comp_bits.a = 1;}
				comp_bits.c1 = 1;
				comp_bits.c2 = 1;
				comp_bits.c3 = 0;
				comp_bits.c4 = 0;
				comp_bits.c5 = 1;
				comp_bits.c6 = 1;
			},
			"D+1" => {
				comp_bits.c1 = 0;
				comp_bits.c2 = 1;
				comp_bits.c3 = 1;
				comp_bits.c4 = 1;
				comp_bits.c5 = 1;
				comp_bits.c6 = 1;
			},
			"A+1" | "M+1" => {
				if mnemonic == "M+1" {comp_bits.a = 1;}
				comp_bits.c1 = 1;
				comp_bits.c2 = 1;
				comp_bits.c3 = 0;
				comp_bits.c4 = 1;
				comp_bits.c5 = 1;
				comp_bits.c6 = 1;
			},
			"D-1" => {
				comp_bits.c1 = 0;
				comp_bits.c2 = 0;
				comp_bits.c3 = 1;
				comp_bits.c4 = 1;
				comp_bits.c5 = 1;
				comp_bits.c6 = 0;
			},
			"A-1" | "M-1" => {
				if mnemonic == "M-1" {comp_bits.a = 1;}
				comp_bits.c1 = 1;
				comp_bits.c2 = 1;
				comp_bits.c3 = 0;
				comp_bits.c4 = 0;
				comp_bits.c5 = 1;
				comp_bits.c6 = 0;
			},
			"D+A" | "D+M" => {
				if mnemonic == "D+M" {comp_bits.a = 1;}
				comp_bits.c1 = 0;
				comp_bits.c2 = 0;
				comp_bits.c3 = 0;
				comp_bits.c4 = 0;
				comp_bits.c5 = 1;
				comp_bits.c6 = 0;
			},
			"D-A" | "D-M" => {
				if mnemonic == "D-M" {comp_bits.a = 1;}
				comp_bits.c1 = 0;
				comp_bits.c2 = 1;
				comp_bits.c3 = 0;
				comp_bits.c4 = 0;
				comp_bits.c5 = 1;
				comp_bits.c6 = 1;
			},
			"A-D" | "M-D" => {
				if mnemonic == "M-D" {comp_bits.a = 1;}
				comp_bits.c1 = 0;
				comp_bits.c2 = 0;
				comp_bits.c3 = 0;
				comp_bits.c4 = 1;
				comp_bits.c5 = 1;
				comp_bits.c6 = 1;
			},
			"D&A" | "D&M" => {
				if mnemonic == "D&M" {comp_bits.a = 1;}
				comp_bits.c1 = 0;
				comp_bits.c2 = 0;
				comp_bits.c3 = 0;
				comp_bits.c4 = 0;
				comp_bits.c5 = 0;
				comp_bits.c6 = 0;
			},
			"D|A" | "D|M" => {
				if mnemonic == "D|M" {comp_bits.a = 1;}
				comp_bits.c1 = 0;
				comp_bits.c2 = 1;
				comp_bits.c3 = 0;
				comp_bits.c4 = 1;
				comp_bits.c5 = 0;
				comp_bits.c6 = 1;
			},
			_ => {},
		}
		comp_bits
	}

	/// Returns the binary code of the jump mnemonic
	///
	/// returns 3 bits
	fn jump(mnemonic: &str) -> Jump {
		let mut jump_bits = Jump::new();
		match mnemonic {
			"null" 	=> {},
			"JGT"	=> {
				jump_bits.j3 = 1;
			},
			"JEQ"	=> {
				jump_bits.j2 = 1;
			},
			"JGE"	=> {
				jump_bits.j3 = 1;
				jump_bits.j2 = 1;
			},
			"JLT"	=> {
				jump_bits.j1 = 1;
			},
			"JNE"	=> {
				jump_bits.j1 = 1;
				jump_bits.j3 = 1;
			},
			"JLE"	=> {
				jump_bits.j1 = 1;
				jump_bits.j2 = 1;
			},
			"JMP"	=> {
				jump_bits.j1 = 1;
				jump_bits.j2 = 1;
				jump_bits.j3 = 1;
			},
			_		=> {},
		}
		jump_bits
	}
}

/// Change the bits at indexes in index to values in dest
///
/// Takes a hashmap such as bit_index -> bit_value
#[allow(dead_code)]
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

struct Dest {
	d1: u8,
	d2: u8,
	d3: u8,
}

impl Dest {
	fn new() -> Dest {
		Dest {
			d1: 0,
			d2: 0,
			d3: 0,
		}
	}
}

impl fmt::Display for Dest {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{0}{1}{2}",self.d1, self.d2, self.d3)
	}
}

struct Comp {
	a: u8,
	c1: u8,
	c2: u8,
	c3: u8,
	c4: u8,
	c5: u8,
	c6: u8,
}

impl Comp {
	fn new() -> Comp {
		Comp {
			a: 0,
			c1: 0,
			c2: 0,
			c3: 0,
			c4: 0,
			c5: 0,
			c6: 0,
		}
	}
}

impl fmt::Display for Comp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{0}{1}{2}{3}{4}{5}{6}",
			self.a, self.c1, self.c2, self.c3,self.c4,self.c5, self.c6)
	}
}

struct Jump {
	j1: u8,
	j2: u8,
	j3: u8,
}

impl Jump {
	fn new() -> Jump {
		Jump {
			j1: 0,
			j2: 0,
			j3: 0,
		}
	}
}

impl fmt::Display for Jump {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{0}{1}{2}",self.j1, self.j2, self.j3)
	}
}

/// Keeps a correspondance between symbolic label and numeric addresses.
///
/// Hack instructions can contain symbols that each must be resolved into
/// actual addresses as part of the translation process.
struct SymbolTable {
	table: HashMap<String, u16>,
}

impl SymbolTable {
	/// Creates a new empty symbol table
	fn new() -> SymbolTable {
		SymbolTable {
			table: HashMap::new(),
		}
	}

	/// Initialise the symbol table with predefined symbols.
	fn initialise(&mut self) {
		self.table.insert("SP".to_string(), 0);
		self.table.insert("LCL".to_string(), 1);
		self.table.insert("ARG".to_string(), 2);
		self.table.insert("THIS".to_string(), 3);
		self.table.insert("THAT".to_string(), 4);
		for i in 0..16 {
			self.table.insert(format!("R{}", i).to_string(), i);
		}
		self.table.insert("SCREEN".to_string(), 16384);
		self.table.insert("KBD".to_string(), 24576);
	}

	/// Adds the pair (symbol, address) to the table
	fn add_entry(&mut self, symbol: String, address: u16) {
		self.table.insert(symbol, address);
	}

	/// Determines whether the symbol table contain the given symbol
	fn contains(&self, symbol: &str) -> bool {
		self.table.contains_key(symbol)
	}

	/// Returns the address associated with the `symbol`.
	fn get_address(&self, symbol: &str) -> u16 {
		let tmp = self.table.get(symbol).unwrap();
		tmp.clone()
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn it_works() {
// 		assert_eq!(false, true);
// 	}	
// }