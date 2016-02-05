extern crate rustc_serialize;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
	let path = env::args().last().unwrap();

	let mut f = match File::open(&path) {
		Err(e) => panic!("Unable to open {}: {}", path, Error::description(&e)),
		Ok(file) => file,
	};

	let mut fusion_tools_string = String::new();

	match f.read_to_string(&mut fusion_tools_string) {
		Err(e) => panic!("Unable to read {}: {}", path, Error::description(&e)),
		Ok(string) => string,
	};

	print!("{}", fusion_tools_string);
}