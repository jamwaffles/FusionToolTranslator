extern crate rustc_serialize;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
	let path = Path::new("./tools.json");

	let mut f = match File::open("./tools.json") {
		Err(e) => panic!("Unable to open {}: {}", path.display(), Error::description(&e)),
		Ok(file) => file,
	};

	let mut fusionToolsString = String::new();

	match f.read_to_string(&mut fusionToolsString) {
		Err(e) => panic!("Unable to read {}: {}", path.display(), Error::description(&e)),
		Ok(string) => string,
	};

	print!("{}", fusionToolsString);
}
