use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::env;


// OWNERSHIP: we cannot use a binding after we’ve moved it aka taken a reference to it or passed it as an args
// 
//


// enum CommandType {
// 	A_COMMAND,
// 	C_COMMAND,
// 	L_COMMAND,
// }
struct Parser<'a> {
	commands: Vec<&'a str>,
}

impl<'a> Parser<'a> {
	// remove all white space and comments
	fn new(asm_filename: &str) -> Parser {
		let asm_file_content = load_asm(asm_filename);
		Parser {
			commands: asm_file_content.trim().split('\n').collect(),
		}
	}
}

fn load_asm(filename: &str) -> String {
	//create a path to the desired file
	let path = Path::new(filename);
	let display = path.display();

	// open the path in read-only mode
	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
		Ok(file) => file,
	};

	// Read the file content into a string
	let mut file_content = String::new();
	match file.read_to_string(&mut file_content) {
		Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
		Ok(_) => print!("{} contains:\n{}", display, file_content),
	}
	file_content
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
