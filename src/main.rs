extern crate rustc_serialize;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;

fn main() {
	let path = env::args().last().unwrap();

	let mut f = match File::open(&path) {
		Err(e) => panic!("Unable to open {}: {}", path, Error::description(&e)),
		Ok(file) => file,
	};

	let mut fusionToolsString = String::new();

	match f.read_to_string(&mut fusionToolsString) {
		Err(e) => panic!("Unable to read {}: {}", path, Error::description(&e)),
		Ok(string) => string,
	};

	print!("{}", fusionToolsString);
}
