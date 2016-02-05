extern crate rustc_serialize;
use rustc_serialize::json::Json;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
	let path = env::args().last().unwrap();

	let mut f = match File::open(&path) {
		Ok(file) => file,
		Err(e) => panic!("Unable to open {}: {}", path, Error::description(&e)),
	};

	let mut fusion_tools_string = String::new();

	match f.read_to_string(&mut fusion_tools_string) {
		Ok(string) => string,
		Err(e) => panic!("Unable to read {}: {}", path, Error::description(&e)),
	};

	print!("{}", fusion_tools_string);

	let parsed_file = match Json::from_str(&fusion_tools_string) {
		Ok(parsed) => parsed,
		Err(e) => panic!("Invalid/empty JSON: {}", Error::description(&e)),
	};

	let fusion_tools = parsed_file.find_path(&[ "data" ]).unwrap().as_array().unwrap();

	print!("{:?}", fusion_tools);
}