extern crate rustc_serialize;
extern crate zip;

mod fusion_tool;
mod linuxcnc_tool;

use std::error::Error;
use std::fs::{ File, OpenOptions };
use std::io::{ Read, Write };
use std::env;
use std::process;
use std::path;

use rustc_serialize::json::Json;
use fusion_tool::FusionTool;
use linuxcnc_tool::LinuxCNCTool;

fn fusion360_convert(fusion_tools_json: String) -> Vec<LinuxCNCTool> {
	let parsed_file = match Json::from_str(&fusion_tools_json) {
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
		.filter_map(|tool| {
			let desc = match tool.find("description") {
				Some(field) => match field.as_string() {
					Some(description) => String::from(description),
					None => panic!("Tool description parse error"),
				},
				None => String::from("(no description)"),
			};

			let tool_number = match tool.find_path(&[ "post-process", "number" ]) {
				Some(field) => match field.as_u64() {
					Some(number) => Some(number as u16),
					None => panic!("Tool number parse error, probably not a number"),
				},
				None => None,
			};

			match tool_number {
				Some(number) =>
					Some(FusionTool {
						number: number,

						description: desc.clone(),

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
								None => panic!("Tool diameter is not a number for tool #{}", number),
							},
							None => panic!("Tool diameter not defined {}", desc),
						}

				}),

				None => {
					println!("WARN: Tool has no post, ignoring tool {}", desc);

					None
				},
			}
		})
		.collect::<Vec<FusionTool>>();

	println!("{} tools parsed\n", fusion_tools.len());

	println!("No.\tDia.\tDescription");

	for tool in fusion_tools.iter() {
		println!("{}", tool);
	}

	println!("");

	fusion_tools
		.iter()
		.map(|tool| LinuxCNCTool {
			number: tool.number,
			pocket: tool.number,
			diameter: tool.diameter,
			description: format!("{} ({})", tool.description, tool.family),
		})
		.collect::<Vec<LinuxCNCTool>>()
}

fn main() {
	if env::args().len() < 3 {
		panic!("Program must at least have an input and output path");
	}

	let out_path_arg = env::args().last().unwrap();
	let path_arg = env::args().nth(env::args().len() - 2).unwrap();

	let out_path = path::Path::new(&out_path_arg);
	let in_path = path::Path::new(&path_arg);

	let f = match File::open(&in_path) {
		Ok(file) => file,
		Err(e) => panic!("Unable to open {}: {}", in_path.display(), Error::description(&e)),
	};

	let mut zip = zip::read::ZipArchive::new(f).unwrap();

	let mut zip_file = zip.by_index(0).unwrap();

	let mut fusion_tools_string = String::new();

	match zip_file.read_to_string(&mut fusion_tools_string) {
		Ok(string) => string,
		Err(e) => panic!("Unable to read {}: {}", in_path.display(), Error::description(&e)),
	};

	let linuxcnc_tools = fusion360_convert(fusion_tools_string);

	let mut output_file = match OpenOptions::new().write(true).create(true).open(&out_path) {
		Ok(file) => file,
		Err(e) => panic!("Unable to open {} for writing: {}", out_path.display(), Error::description(&e)),
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
		Ok(_) => println!("Tool table saved to {} successfully", out_path.display()),
		Err(e) => panic!("JSON parse error: {}", Error::description(&e)),
	};
}