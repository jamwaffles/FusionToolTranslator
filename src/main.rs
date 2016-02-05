extern crate rustc_serialize;
mod fusion_tool;
mod linuxcnc_tool;

use std::error::Error;
use std::fs::{ File, OpenOptions };
use std::io::{ Read, Write };
use std::env;
use std::process;

use rustc_serialize::json::Json;
use fusion_tool::FusionTool;
use linuxcnc_tool::LinuxCNCTool;

fn main() {
	let path = env::args().last().unwrap();
	let out_path = "./tooltable.tbl";

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

	let fusion_tools = parsed_tools
		.iter()
		.map(|tool| {
			let tool_number = match tool.find_path(&[ "post-process", "number" ]) {
				Some(field) => match field.as_u64() {
					Some(number) => number as u16,
					None => panic!("Tool number parse error, probably not a number"),
				},
				None => panic!("Tool number not defined"),
			};

			FusionTool {
				number: tool_number,

				description: match tool.find("description") {
					Some(field) => match field.as_string() {
						Some(description) => String::from(description),
						None => panic!("Tool description parse error"),
					},
					None => String::from(""),
				},

				family: match tool.find("type") {
					Some(field) => match field.as_string() {
						Some(family) => String::from(family),
						None => panic!("Tool type parse error"),
					},
					None => String::from(""),
				},

				diameter: match tool.find_path(&[ "geometry", "DC" ]) {
					Some(field) => match field.as_f64() {
						Some(number) => number as f32,
						None => panic!("Tool diameter is not a number for tool #{}", tool_number),
					},
					None => panic!("Tool diameter not defined"),
				}
			}
		})
		.collect::<Vec<FusionTool>>();

	println!("{} tools imported from Fusion 360\n", fusion_tools.len());

	println!("No.\tDia.\tDescription");

	for tool in fusion_tools.iter() {
		println!("{}", tool);
	}

	println!("");

	let linuxcnc_tools = fusion_tools
		.iter()
		.map(|tool| LinuxCNCTool {
			number: tool.number,
			pocket: tool.number,
			diameter: tool.diameter,
			description: format!("{} ({})", tool.description, tool.family),
		})
		.collect::<Vec<LinuxCNCTool>>();

	let mut output_file = match OpenOptions::new().write(true).create(true).open(out_path) {
		Ok(file) => file,
		Err(e) => panic!("Unable to open {} for writing: {}", out_path, Error::description(&e)),
	};

	match output_file.set_len(0) {
		Err(e) => panic!("Could not reset tool table file: {}", Error::description(&e)),
		_ => ()
	};

	for t in linuxcnc_tools.iter() {
		let line = format!("T{} P{} D{} ;{}\n", t.number, t.number, t.diameter, t.description);

		match output_file.write(String::from(line).as_bytes()) {
			Err(e) => panic!("Could not write tool table record: {}", Error::description(&e)),
			_ => ()
		};
	}

	match output_file.sync_all() {
		Ok(_) => println!("Tool table saved successfully"),
		Err(e) => panic!("JSON parse error: {}", Error::description(&e)),
	};
}