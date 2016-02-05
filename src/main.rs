extern crate rustc_serialize;
mod fusion_tool;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::env;
use std::process;

use rustc_serialize::json::Json;
use fusion_tool::FusionTool;

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

	let parsed_file = match Json::from_str(&fusion_tools_string) {
		Ok(parsed) => parsed,
		Err(e) => panic!("JSON parse error: {}", Error::description(&e)),
	};

	let parsed_tools = match parsed_file.find("data") {
		Some(list) => match list.as_array() {
			Some(tools) => tools,
			None => {
				println!("No tools found to convert");
				process::exit(0);
			}
		},
		None => panic!("Invalid JSON: 'data' key not present"),
	};

	let mut fusion_tools: Vec<FusionTool> = Vec::new();

	for tool in parsed_tools.iter() {
		fusion_tools.push(FusionTool {
			number: tool.find_path(&[ "post-process", "number" ]).unwrap().as_u64().unwrap() as u16,
			pocket: tool.find_path(&[ "post-process", "number" ]).unwrap().as_u64().unwrap() as u16,
			description: String::from(tool.find("description").unwrap().as_string().unwrap()),
			diameter: tool.find_path(&[ "geometry", "tip-diameter" ]).unwrap().as_f64().unwrap() as f32,
		})
	}

	println!("{}", fusion_tools[0]);
}